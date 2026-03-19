#!/usr/bin/env python3
import subprocess
import sys
import os

def build_executable():
    print("Building LLM Sidecar executable...")
    
    cmd = [
        sys.executable, "-m", "PyInstaller",
        "--onefile",
        "--name", "llm_sidecar",
        "--console",
        "--collect-all", "uvicorn",
        "--collect-all", "fastapi", 
        "--collect-all", "openai",
        "--collect-all", "anthropic",
        "--collect-all", "pydantic",
        "--collect-all", "httpx",
        "--collect-all", "websockets",
        "api.py"
    ]
    
    result = subprocess.run(cmd, cwd=os.path.dirname(os.path.abspath(__file__)))
    
    if result.returncode == 0:
        print("\nBuild successful!")
        print("Executable location: dist/llm_sidecar.exe")
    else:
        print("\nBuild failed!")
        sys.exit(1)

if __name__ == "__main__":
    build_executable()
