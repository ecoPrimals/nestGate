#!/usr/bin/env python3
"""
Real HuggingFace Model Downloader
Actually downloads models from HuggingFace using the API key!
"""

import os
import sys
from pathlib import Path

try:
    from huggingface_hub import hf_hub_download, HfApi
    import toml
except ImportError:
    print("📦 Installing required packages...")
    import subprocess
    subprocess.check_call([sys.executable, "-m", "pip", "install", "-q", "huggingface_hub", "toml"])
    from huggingface_hub import hf_hub_download, HfApi
    import toml

# Load API key from testing-secrets  
SECRETS_PATH = Path("/home/eastgate/Development/ecoPrimals/testing-secrets/api-keys.toml")

def load_huggingface_token():
    """Load HuggingFace token from secrets file."""
    try:
        with open(SECRETS_PATH) as f:
            content = f.read()
            # Simple parsing for the token
            for line in content.split('\n'):
                if 'hugging face' in line.lower():
                    continue
                line = line.strip()
                if line and not line.startswith('#') and not line.startswith('['):
                    return line
    except Exception as e:
        print(f"⚠️  Could not load token: {e}")
        return None

def download_model(repo_id: str, filename: str, token: str, cache_dir: str = "/tmp/nestgate_models"):
    """
    Download a specific file from HuggingFace.
    
    Args:
        repo_id: Repository ID (e.g., "facebook/esm2_t6_8M_UR50D")
        filename: File to download
        token: HuggingFace API token
        cache_dir: Local cache directory
    """
    print(f"📥 Downloading {repo_id}/{filename}...")
    print(f"   Cache: {cache_dir}")
    
    try:
        path = hf_hub_download(
            repo_id=repo_id,
            filename=filename,
            token=token,
            cache_dir=cache_dir,
            resume_download=True
        )
        
        # Get file size
        size_mb = os.path.getsize(path) / (1024 * 1024)
        
        print(f"✅ Downloaded successfully!")
        print(f"   Path: {path}")
        print(f"   Size: {size_mb:.2f} MB")
        
        return path
        
    except Exception as e:
        print(f"❌ Error: {e}")
        return None

def list_models():
    """List available small models for demo."""
    models = [
        {
            "repo": "facebook/esm2_t6_8M_UR50D",
            "file": "pytorch_model.bin",
            "size_mb": 30,
            "description": "ESM-2 Protein Model (8M params)"
        },
        {
            "repo": "sentence-transformers/all-MiniLM-L6-v2",
            "file": "pytorch_model.bin",
            "size_mb": 80,
            "description": "Sentence Transformer (22M params)"
        },
        {
            "repo": "distilbert-base-uncased",
            "file": "pytorch_model.bin",
            "size_mb": 250,
            "description": "DistilBERT (66M params)"
        }
    ]
    
    print("\n📚 Available Models for Demo:")
    print("═" * 60)
    for i, model in enumerate(models, 1):
        print(f"\n{i}. {model['description']}")
        print(f"   Repo: {model['repo']}")
        print(f"   File: {model['file']}")
        print(f"   Size: ~{model['size_mb']} MB")
    print("\n" + "═" * 60)
    
    return models

def main():
    """Main demo function."""
    print("🤖 ================================================")
    print("🤖  REAL HUGGINGFACE MODEL DOWNLOADER")
    print("🤖  Actual API Integration with NestGate")
    print("🤖 ================================================")
    print()
    
    # Load token
    token = load_huggingface_token()
    if not token:
        print("❌ Could not load HuggingFace token from testing-secrets")
        print("   Expected location:", SECRETS_PATH)
        return
    
    print(f"✅ HuggingFace token loaded")
    print(f"   Token: {token[:10]}...{token[-10:]}")
    print()
    
    # List available models
    models = list_models()
    
    # Download a small model for demo
    print("\n🎯 Downloading demo model (ESM-2 Protein Model)...")
    print()
    
    model = models[0]
    path = download_model(
        repo_id=model['repo'],
        filename=model['file'],
        token=token
    )
    
    if path:
        print()
        print("🎉 SUCCESS! Model downloaded via real HuggingFace API!")
        print()
        print("📊 NestGate Storage Benefits:")
        print("   • Automatic compression (LZ4)")
        print("   • Checksum verification (Blake3)")
        print("   • Version snapshots")
        print("   • Smart caching")
        print("   • Fast local access")
        print()
        print("🔄 In production:")
        print(f"   $ nestgate put ml/models/{model['repo'].split('/')[-1]}/ {path}")
        print(f"   $ nestgate snapshot create ml/models/{model['repo'].split('/')[-1]}@downloaded")
        print()
        print("✅ Model ready for use with NestGate!")
    else:
        print()
        print("⚠️  Download failed, but the demo still works!")

if __name__ == "__main__":
    main()

