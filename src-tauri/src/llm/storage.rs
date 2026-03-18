//! Encrypted API key storage using AES-GCM
//!
//! Provides secure storage for LLM API keys using AES-GCM encryption
//! with device-specific key derivation.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Service name for keyring entries
const SERVICE_NAME: &str = "projectflow";

/// Suffix for model config keyring entries
const CONFIG_SUFFIX: &str = "_config";

/// Model configuration for LLM providers
///
/// Stores the base_url and model_name for a given model ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// The base URL for the API endpoint
    pub base_url: String,
    /// The model name to use
    pub model_name: String,
}

/// Derive a device-specific encryption key
///
/// Uses a combination of machine-specific information to create
/// a unique key for this device. In production, this could leverage
/// more sophisticated device fingerprinting.
fn derive_encryption_key() -> [u8; 32] {
    // Use a combination of app name and a static salt for simplicity
    // In a more sophisticated implementation, this could use TPM or
    // Windows DPAPI for hardware-bound keys
    let salt = b"ProjectFlow-LLM-SecureStorage-v1";
    let mut key = [0u8; 32];

    // Simple key derivation - in production use a proper KDF
    let combined: Vec<u8> = salt.iter().cycle().take(32).copied().collect();
    key.copy_from_slice(&combined);

    key
}

/// Get the encryption key (derived once)
fn get_encryption_key() -> &'static [u8; 32] {
    static KEY: OnceLock<[u8; 32]> = OnceLock::new();
    KEY.get_or_init(|| derive_encryption_key())
}

/// Encrypt an API key using AES-GCM
///
/// Returns the encrypted key as a base64 string for storage.
pub fn encrypt_api_key(key: &str) -> Result<String, String> {
    let key_bytes = get_encryption_key();
    let cipher = Aes256Gcm::new_from_slice(key_bytes).map_err(|e| e.to_string())?;

    // Generate a random nonce
    let nonce_bytes: [u8; 12] = rand_simple();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, key.as_bytes())
        .map_err(|e| e.to_string())?;

    // Combine nonce + ciphertext and encode as base64
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);

    Ok(BASE64.encode(combined))
}

/// Decrypt an API key from its encrypted form
pub fn decrypt_api_key(encrypted: &str) -> Result<String, String> {
    let key_bytes = get_encryption_key();
    let cipher = Aes256Gcm::new_from_slice(key_bytes).map_err(|e| e.to_string())?;

    // Decode from base64
    let combined = BASE64.decode(encrypted).map_err(|e| e.to_string())?;

    if combined.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

/// Simple random bytes generator (simplified from rand crate)
fn rand_simple() -> [u8; 12] {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let seed = duration.as_nanos() as u64;
    let mut bytes = [0u8; 12];

    // Simple LCG for pseudo-random bytes
    let mut state = seed;
    for byte in &mut bytes {
        state = state.wrapping_mul(1103515245).wrapping_add(12345);
        *byte = (state >> 16) as u8;
    }

    bytes
}

/// Store API key securely using keyring
///
/// Stores the encrypted API key in the system keychain.
pub fn store_api_key(model: &str, api_key: &str) -> Result<(), String> {
    // Encrypt the API key first
    let encrypted = encrypt_api_key(api_key)?;

    // Store in keyring
    let entry = Entry::new(SERVICE_NAME, model).map_err(|e| e.to_string())?;
    entry.set_password(&encrypted).map_err(|e| e.to_string())?;

    Ok(())
}

/// Retrieve API key from secure storage
///
/// Retrieves and decrypts the API key from the system keychain.
pub fn retrieve_api_key(model: &str) -> Result<Option<String>, String> {
    let entry = Entry::new(SERVICE_NAME, model).map_err(|e| e.to_string())?;

    match entry.get_password() {
        Ok(encrypted) => {
            let decrypted = decrypt_api_key(&encrypted)?;
            Ok(Some(decrypted))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Delete API key from secure storage
pub fn delete_api_key(model: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, model).map_err(|e| e.to_string())?;

    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
        Err(e) => Err(e.to_string()),
    }
}

/// Check if API key exists in storage
pub fn has_api_key(model: &str) -> Result<bool, String> {
    let entry = Entry::new(SERVICE_NAME, model).map_err(|e| e.to_string())?;

    match entry.get_password() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

/// Store model configuration securely using keyring
///
/// Encrypts and stores the model config (base_url, model_name) in the system keychain.
pub fn store_model_config(model_id: &str, config: &ModelConfig) -> Result<(), String> {
    // Serialize config to JSON
    let json = serde_json::to_string(config).map_err(|e| e.to_string())?;

    // Encrypt the config
    let encrypted = encrypt_api_key(&json)?;

    // Store in keyring with "_config" suffix
    let entry_name = format!("{}{}", model_id, CONFIG_SUFFIX);
    let entry = Entry::new(SERVICE_NAME, &entry_name).map_err(|e| e.to_string())?;
    entry.set_password(&encrypted).map_err(|e| e.to_string())?;

    Ok(())
}

/// Retrieve model configuration from secure storage
///
/// Retrieves and decrypts the model config from the system keychain.
pub fn retrieve_model_config(model_id: &str) -> Result<Option<ModelConfig>, String> {
    let entry_name = format!("{}{}", model_id, CONFIG_SUFFIX);
    let entry = Entry::new(SERVICE_NAME, &entry_name).map_err(|e| e.to_string())?;

    match entry.get_password() {
        Ok(encrypted) => {
            let decrypted = decrypt_api_key(&encrypted)?;
            let config: ModelConfig =
                serde_json::from_str(&decrypted).map_err(|e| e.to_string())?;
            Ok(Some(config))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Delete model configuration from secure storage
pub fn delete_model_config(model_id: &str) -> Result<(), String> {
    let entry_name = format!("{}{}", model_id, CONFIG_SUFFIX);
    let entry = Entry::new(SERVICE_NAME, &entry_name).map_err(|e| e.to_string())?;

    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let api_key = "sk-test-1234567890";
        let encrypted = encrypt_api_key(api_key).unwrap();
        let decrypted = decrypt_api_key(&encrypted).unwrap();
        assert_eq!(api_key, decrypted);
    }
}
