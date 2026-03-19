from anthropic import AsyncAnthropic
from typing import List, Dict, Any, Optional, AsyncIterator
import logging

logger = logging.getLogger(__name__)


class AnthropicClient:
    def __init__(self, api_key: str):
        self.client = AsyncAnthropic(api_key=api_key)

    async def chat_completions(
        self,
        messages: List[Dict[str, str]],
        model: str = "claude-3-5-sonnet-20241022",
        temperature: float = 0.7,
        max_tokens: Optional[int] = None,
        stream: bool = False
    ) -> Dict[str, Any] | AsyncIterator[str]:
        try:
            system_message = ""
            conversation_messages = []
            
            for msg in messages:
                if msg["role"] == "system":
                    system_message = msg["content"]
                else:
                    conversation_messages.append(msg)
            
            if stream:
                return self._stream_chat(
                    system_message,
                    conversation_messages,
                    model,
                    temperature,
                    max_tokens
                )
            else:
                return await self._non_stream_chat(
                    system_message,
                    conversation_messages,
                    model,
                    temperature,
                    max_tokens
                )
        except Exception as e:
            logger.error(f"Anthropic API error: {e}")
            raise

    async def _non_stream_chat(
        self,
        system_message: str,
        messages: List[Dict[str, str]],
        model: str,
        temperature: float,
        max_tokens: Optional[int]
    ) -> Dict[str, Any]:
        response = await self.client.messages.create(
            model=model,
            system=system_message if system_message else None,
            messages=messages,
            temperature=temperature,
            max_tokens=max_tokens or 1024,
            stream=False
        )

        content = ""
        for block in response.content:
            if hasattr(block, 'text'):
                content += block.text

        return {
            "content": content,
            "model": response.model,
            "provider": "anthropic",
            "usage": {
                "input_tokens": response.usage.input_tokens,
                "output_tokens": response.usage.output_tokens
            }
        }

    async def _stream_chat(
        self,
        system_message: str,
        messages: List[Dict[str, str]],
        model: str,
        temperature: float,
        max_tokens: Optional[int]
    ) -> AsyncIterator[str]:
        async with self.client.messages.stream(
            model=model,
            system=system_message if system_message else None,
            messages=messages,
            temperature=temperature,
            max_tokens=max_tokens or 1024
        ) as stream:
            async for chunk in stream:
                if chunk.type == "content_block_delta":
                    if chunk.delta.type == "text_delta":
                        yield chunk.delta.text

    async def list_models(self) -> List[Dict[str, Any]]:
        return [
            {"id": "claude-3-5-sonnet-20241022", "name": "Claude 3.5 Sonnet"},
            {"id": "claude-3-opus-20240229", "name": "Claude 3 Opus"},
            {"id": "claude-3-haiku-20240307", "name": "Claude 3 Haiku"}
        ]

    async def close(self):
        pass
