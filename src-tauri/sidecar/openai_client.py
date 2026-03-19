from openai import AsyncOpenAI
from typing import List, Dict, Any, Optional, AsyncIterator
import logging

logger = logging.getLogger(__name__)


class OpenAIClient:
    def __init__(self, api_key: str, api_url: str = "https://api.openai.com/v1"):
        self.client = AsyncOpenAI(
            api_key=api_key,
            base_url=api_url
        )
        self.api_url = api_url

    async def chat_completions(
        self,
        messages: List[Dict[str, str]],
        model: str = "gpt-4o-mini",
        temperature: float = 0.7,
        max_tokens: Optional[int] = None,
        stream: bool = False
    ) -> Dict[str, Any] | AsyncIterator[str]:
        try:
            if stream:
                return self._stream_chat(messages, model, temperature, max_tokens)
            else:
                return await self._non_stream_chat(messages, model, temperature, max_tokens)
        except Exception as e:
            logger.error(f"OpenAI API error: {e}")
            raise

    async def _non_stream_chat(
        self,
        messages: List[Dict[str, str]],
        model: str,
        temperature: float,
        max_tokens: Optional[int]
    ) -> Dict[str, Any]:
        response = await self.client.chat.completions.create(
            model=model,
            messages=messages,
            temperature=temperature,
            max_tokens=max_tokens,
            stream=False
        )

        return {
            "content": response.choices[0].message.content,
            "model": response.model,
            "provider": "openai",
            "usage": {
                "prompt_tokens": response.usage.prompt_tokens,
                "completion_tokens": response.usage.completion_tokens,
                "total_tokens": response.usage.total_tokens
            } if response.usage else None
        }

    async def _stream_chat(
        self,
        messages: List[Dict[str, str]],
        model: str,
        temperature: float,
        max_tokens: Optional[int]
    ) -> AsyncIterator[str]:
        async with self.client.chat.completions.create(
            model=model,
            messages=messages,
            temperature=temperature,
            max_tokens=max_tokens,
            stream=True
        ) as stream:
            async for chunk in stream:
                if chunk.choices[0].delta.content:
                    yield chunk.choices[0].delta.content

    async def list_models(self) -> List[Dict[str, Any]]:
        try:
            response = await self.client.models.list()
            return [{"id": m.id} for m in response.data]
        except Exception as e:
            logger.error(f"Failed to list models: {e}")
            return []

    async def close(self):
        await self.client.close()
