<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";
import Button from "../../ui/Button.vue";
import { useLlmStore, DEFAULT_MODEL_CONFIGS } from "../../../stores/llmStore";
import { type ModelConfig } from "../../../lib/api";

interface Props {
  open: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const llmStore = useLlmStore();

const selectedModel = ref("gpt-4o");
const apiKey = ref("");
const baseUrl = ref("");
const modelName = ref("");
const loading = ref(false);

const models = computed(() => {
  return Object.entries(DEFAULT_MODEL_CONFIGS).map(([id, config]) => ({
    id,
    name: `${config.name} (${config.default_model})`,
  }));
});

onMounted(() => {
  selectedModel.value = llmStore.selectedModelId || llmStore.selectedModel;
  loadModelConfig();
});

watch(selectedModel, () => {
  loadModelConfig();
});

function loadModelConfig() {
  const config = llmStore.modelConfig;
  if (config && config.model_id === selectedModel.value) {
    baseUrl.value = config.base_url;
    modelName.value = config.model_name;
    apiKey.value = config.api_key;
  } else {
    // Use defaults
    const defaults = DEFAULT_MODEL_CONFIGS[selectedModel.value];
    if (defaults) {
      baseUrl.value = defaults.base_url;
      modelName.value = defaults.default_model;
      apiKey.value = "";
    }
  }
}

async function saveKey() {
  if (!apiKey.value.trim()) return;

  loading.value = true;
  try {
    // Save model config with base_url, model_name, and api_key
    const config: ModelConfig = {
      model_id: selectedModel.value,
      base_url: baseUrl.value,
      model_name: modelName.value,
      api_key: apiKey.value,
    };
    console.log("[LlmSettings] Saving - selectedModel:", selectedModel.value, "config:", config);
    await llmStore.saveModelConfig(config);
    
    // Also update the store's selected model ID and selected model
    llmStore.selectedModelId = selectedModel.value;
    llmStore.selectedModel = selectedModel.value;
    
    // Close the dialog after successful save
    emit("close");
  } catch (e) {
    console.error("Failed to save API key:", e);
  } finally {
    loading.value = false;
  }
}

function close() {
  emit("close");
}
</script>

<template>
  <Modal :open="open" title="API 设置" size="sm" @close="close">
    <div class="space-y-4">
      <!-- Model selector -->
      <div>
        <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
          模型
        </label>
        <select
          v-model="selectedModel"
          class="w-full bg-[var(--bg-tertiary)] border border-[var(--border-default)] rounded-lg px-3 py-2.5 text-sm text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)]"
        >
          <option v-for="model in models" :key="model.id" :value="model.id">
            {{ model.name }}
          </option>
        </select>
      </div>

      <!-- Base URL input -->
      <div>
        <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
          Base URL
        </label>
        <Input
          v-model="baseUrl"
          type="text"
          placeholder="https://api.example.com/v1"
        />
      </div>

      <!-- Model Name input -->
      <div>
        <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
          Model Name
        </label>
        <Input
          v-model="modelName"
          type="text"
          placeholder="model-name"
        />
      </div>

      <!-- API Key input -->
      <div>
        <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
          API Key
        </label>
        <Input
          v-model="apiKey"
          type="password"
          placeholder="sk-..."
        />
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-2">
        <Button variant="secondary" size="sm" @click="close">
          取消
        </Button>
        <Button
          variant="primary"
          size="sm"
          :loading="loading"
          :disabled="!apiKey.trim()"
          @click="saveKey"
        >
          保存
        </Button>
      </div>
    </template>
  </Modal>
</template>
