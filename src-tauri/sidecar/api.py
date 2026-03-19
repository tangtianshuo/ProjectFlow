from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from typing import Dict, Any, List, Optional
import logging
from contextlib import asynccontextmanager

from config import LlmConfig, ChatRequest, ChatResponse, ConfigUpdateRequest, LLMProvider
from openai_client import OpenAIClient
from anthropic_client import AnthropicClient

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

llm_config = LlmConfig()
openai_client: Optional[OpenAIClient] = None
anthropic_client: Optional[AnthropicClient] = None


def get_client() -> tuple:
    global openai_client, anthropic_client
    
    provider_value = llm_config.provider.value if isinstance(llm_config.provider, LLMProvider) else llm_config.provider
    
    if provider_value == "openai":
        if not openai_client or openai_client.api_url != llm_config.api_url:
            openai_client = OpenAIClient(llm_config.api_key, llm_config.api_url)
        return openai_client, "openai"
    elif provider_value == "anthropic":
        if not anthropic_client:
            anthropic_client = AnthropicClient(llm_config.api_key)
        return anthropic_client, "anthropic"
    elif provider_value == "openai-compatible":
        if not openai_client:
            openai_client = OpenAIClient(llm_config.api_key, llm_config.api_url)
        return openai_client, "openai-compatible"
    else:
        raise HTTPException(status_code=400, detail="Invalid provider")


@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("LLM Sidecar server started")
    yield
    logger.info("LLM Sidecar server shutting down")
    if openai_client:
        await openai_client.close()


app = FastAPI(title="LLM Sidecar API", lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health_check():
    return {"status": "healthy", "provider": llm_config.provider.value}


@app.get("/config")
async def get_config():
    return {
        "provider": llm_config.provider.value,
        "api_url": llm_config.api_url,
        "model": llm_config.model,
        "has_api_key": bool(llm_config.api_key)
    }


@app.post("/config")
async def update_config(request: ConfigUpdateRequest):
    global llm_config, openai_client, anthropic_client
    
    if request.provider is not None and request.provider != "":
        llm_config.provider = LLMProvider(request.provider)
    if request.api_key is not None:
        llm_config.api_key = request.api_key
    if request.api_url is not None and request.api_url != "":
        llm_config.api_url = request.api_url
    if request.model is not None and request.model != "":
        llm_config.model = request.model
    
    if openai_client:
        await openai_client.close()
        openai_client = None
    anthropic_client = None
    
    logger.info(f"Config updated: {llm_config}")
    
    return {
        "provider": llm_config.provider.value if isinstance(llm_config.provider, LLMProvider) else llm_config.provider,
        "api_url": llm_config.api_url,
        "model": llm_config.model
    }


@app.post("/chat")
async def chat(request: ChatRequest):
    try:
        client, provider_name = get_client()
        
        messages = [{"role": msg.role, "content": msg.content} for msg in request.messages]
        
        model = request.model or llm_config.model
        
        if request.stream:
            from fastapi.responses import StreamingResponse
            import json
            
            async def stream_generator():
                async for chunk in client.chat_completions(
                    messages=messages,
                    model=model,
                    temperature=request.temperature,
                    max_tokens=request.max_tokens,
                    stream=True
                ):
                    yield f"data: {json.dumps({'content': chunk})}\n\n"
                yield "data: [DONE]\n\n"
            
            return StreamingResponse(stream_generator(), media_type="text/event-stream")
        else:
            result = await client.chat_completions(
                messages=messages,
                model=model,
                temperature=request.temperature,
                max_tokens=request.max_tokens,
                stream=False
            )
            
            return ChatResponse(**result)
            
    except Exception as e:
        logger.error(f"Chat error: {e}")
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/models")
async def list_models():
    try:
        client, _ = get_client()
        models = await client.list_models()
        return {"models": models}
    except Exception as e:
        logger.error(f"List models error: {e}")
        return {"models": []}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8765)
