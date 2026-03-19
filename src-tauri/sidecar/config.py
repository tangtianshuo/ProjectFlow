from pydantic import BaseModel
from typing import Optional, List, Dict, Any, Union
from enum import Enum


class LLMProvider(str, Enum):
    OPENAI = "openai"
    ANTHROPIC = "anthropic"
    OPENAI_COMPATIBLE = "openai-compatible"


class LlmConfig(BaseModel):
    provider: LLMProvider = LLMProvider.OPENAI
    api_key: str = ""
    api_url: str = "https://api.openai.com/v1"
    model: str = "gpt-4o-mini"


class Message(BaseModel):
    role: str
    content: str


class ChatRequest(BaseModel):
    messages: List[Message]
    model: Optional[str] = None
    temperature: float = 0.7
    max_tokens: Optional[int] = None
    stream: bool = False


class ChatResponse(BaseModel):
    content: str
    model: str
    provider: str
    usage: Optional[Dict[str, int]] = None


class ConfigUpdateRequest(BaseModel):
    provider: Optional[str] = None
    api_key: Optional[str] = None
    api_url: Optional[str] = None
    model: Optional[str] = None
