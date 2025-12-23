# 🚀 Primal Integration Quick Start

**Get your primal connected to NestGate in 10 minutes**

**Date**: November 10, 2025  
**For**: beardog, squirrel, toadstool, songbird, biomeOS teams

---

## ⚡ Ultra Quick Start

```bash
# 1. Start NestGate data service
cd /home/eastgate/Development/ecoPrimals/nestgate/showcase
./scripts/start_data_service.sh --port 9000

# 2. Test connection
curl http://localhost:9000/health

# 3. Get your API key
./scripts/generate_api_key.sh --primal beardog

# 4. Access data
curl http://localhost:9000/api/v1/datasets \
  -H "X-Primal: beardog" \
  -H "X-API-Key: your_key_here"
```

**Done!** You're connected to NestGate's data service.

---

## 📋 **Per-Primal Quick Guides**

### 🔐 beardog (HSM/Security)

```bash
# 1. Get HSM training data (5GB)
curl http://localhost:9000/api/v1/datasets/hsm_training_v1/stream \
  -H "X-Primal: beardog" \
  -H "X-API-Key: $BEARDOG_KEY" \
  --output hsm_training.tar.gz

# 2. Store your HSM model
curl -X POST http://localhost:9000/api/v1/models/beardog/hsm_model \
  -H "X-Primal: beardog" \
  -H "X-API-Key: $BEARDOG_KEY" \
  -F "model=@hsm_v2.safetensors" \
  -F "metadata=@metadata.json"

# 3. Retrieve stored model
curl http://localhost:9000/api/v1/models/beardog/hsm_model/download?version=latest \
  -H "X-Primal: beardog" \
  -H "X-API-Key: $BEARDOG_KEY" \
  --output hsm_model.safetensors
```

**Available Datasets**:
- `hsm_training_v1` - 5GB mixed HSM training data
- `hsm_validation_v1` - 1GB validation set
- `security_test_vectors` - 500MB test data

**Performance Targets**:
- Download: 500 MB/s
- Upload: 300 MB/s
- Latency: <5ms p95

---

### 🧠 squirrel (LLM/AI)

```bash
# 1. Get LLM training corpus (50GB)
curl http://localhost:9000/api/v1/datasets/llm_corpus_v1/stream \
  -H "X-Primal: squirrel" \
  -H "X-API-Key: $SQUIRREL_KEY" \
  --output llm_corpus.tar.gz

# 2. Store LLM checkpoint
curl -X POST http://localhost:9000/api/v1/models/squirrel/llm_base \
  -H "X-Primal: squirrel" \
  -H "X-API-Key: $SQUIRREL_KEY" \
  -F "model=@llm_checkpoint.gguf" \
  -F "metadata=@metadata.json"

# 3. Get tokenized data
curl http://localhost:9000/api/v1/datasets/llm_tokenized_v1/stream \
  -H "X-Primal: squirrel" \
  --output tokenized_data.tar.gz
```

**Available Datasets**:
- `llm_corpus_v1` - 50GB text corpus
- `llm_tokenized_v1` - 35GB pre-tokenized data
- `llm_finetune_v1` - 10GB fine-tuning data

**Performance Targets**:
- Stream: 800 MB/s
- Checkpoint save: <15s (7.5GB model)
- Latency: <10ms p95

---

### 👁️ toadstool (Vision/AI)

```bash
# 1. Get vision training dataset (75GB)
curl http://localhost:9000/api/v1/datasets/vision_training_v1/stream \
  -H "X-Primal: toadstool" \
  -H "X-API-Key: $TOADSTOOL_KEY" \
  --output vision_training.tar.gz

# 2. Store vision model
curl -X POST http://localhost:9000/api/v1/models/toadstool/vision_model \
  -H "X-Primal: toadstool" \
  -H "X-API-Key: $TOADSTOOL_KEY" \
  -F "model=@vision_v1.safetensors" \
  -F "metadata=@metadata.json"

# 3. Get augmented dataset
curl http://localhost:9000/api/v1/datasets/vision_augmented_v1/stream \
  -H "X-Primal: toadstool" \
  --output augmented_data.tar.gz
```

**Available Datasets**:
- `vision_training_v1` - 75GB image dataset
- `vision_augmented_v1` - 100GB augmented images
- `vision_validation_v1` - 15GB validation set

**Performance Targets**:
- Image batch: 1 GB/s
- Model save: <20s (6GB model)
- Latency: <15ms p95

---

### 🎵 songbird (Orchestration)

```bash
# 1. Get orchestration telemetry (25GB)
curl http://localhost:9000/api/v1/datasets/orchestration_telemetry/stream \
  -H "X-Primal: songbird" \
  -H "X-API-Key: $SONGBIRD_KEY" \
  --output telemetry.tar.gz

# 2. Store orchestration model
curl -X POST http://localhost:9000/api/v1/models/songbird/orchestration_model \
  -H "X-Primal: songbird" \
  -H "X-API-Key: $SONGBIRD_KEY" \
  -F "model=@orch_v1.pkl" \
  -F "metadata=@metadata.json"
```

**Available Datasets**:
- `orchestration_telemetry` - 25GB time series
- `service_metrics_v1` - 10GB metrics data
- `load_patterns_v1` - 5GB load testing data

**Performance Targets**:
- Telemetry stream: 600 MB/s
- Real-time updates: <100ms
- Latency: <5ms p95

---

### 🌱 biomeOS (Ecosystem)

```bash
# 1. Get ecosystem integration data (10GB)
curl http://localhost:9000/api/v1/datasets/ecosystem_integration/stream \
  -H "X-Primal: biomeOS" \
  -H "X-API-Key: $BIOMEOS_KEY" \
  --output ecosystem_data.tar.gz

# 2. Store ecosystem state
curl -X POST http://localhost:9000/api/v1/models/biomeOS/ecosystem_state \
  -H "X-Primal: biomeOS" \
  -H "X-API-Key: $BIOMEOS_KEY" \
  -F "model=@state_v1.json" \
  -F "metadata=@metadata.json"
```

**Available Datasets**:
- `ecosystem_integration` - 10GB integration test data
- `service_discovery_v1` - 2GB discovery data
- `config_templates_v1` - 1GB configuration templates

---

## 🔧 **Common Operations**

### List Available Datasets

```bash
curl http://localhost:9000/api/v1/datasets \
  -H "X-Primal: YOUR_PRIMAL"
```

### Get Dataset Metadata

```bash
curl http://localhost:9000/api/v1/datasets/DATASET_ID \
  -H "X-Primal: YOUR_PRIMAL"
```

### Stream Large Dataset (Chunked)

```bash
curl http://localhost:9000/api/v1/datasets/DATASET_ID/stream \
  -H "X-Primal: YOUR_PRIMAL" \
  -H "X-API-Key: YOUR_KEY" \
  --output data.tar.gz \
  --progress-bar
```

### Upload Model

```bash
curl -X POST http://localhost:9000/api/v1/models/YOUR_PRIMAL/MODEL_NAME \
  -H "X-Primal: YOUR_PRIMAL" \
  -H "X-API-Key: YOUR_KEY" \
  -F "model=@model_file.safetensors" \
  -F "metadata=@metadata.json" \
  -F "version=v1.0.0"
```

### List Your Models

```bash
curl http://localhost:9000/api/v1/models/YOUR_PRIMAL \
  -H "X-Primal: YOUR_PRIMAL" \
  -H "X-API-Key: YOUR_KEY"
```

### Download Model

```bash
curl http://localhost:9000/api/v1/models/YOUR_PRIMAL/MODEL_NAME/download \
  -H "X-Primal: YOUR_PRIMAL" \
  -H "X-API-Key: YOUR_KEY" \
  -G -d "version=latest" \
  --output model.safetensors \
  --progress-bar
```

---

## 🐍 **Python Client Example**

```python
import requests
import os

class NestGateClient:
    def __init__(self, primal, api_key, base_url="http://localhost:9000"):
        self.primal = primal
        self.api_key = api_key
        self.base_url = base_url
        self.headers = {
            "X-Primal": primal,
            "X-API-Key": api_key
        }
    
    def list_datasets(self):
        """List available datasets"""
        response = requests.get(
            f"{self.base_url}/api/v1/datasets",
            headers=self.headers
        )
        return response.json()
    
    def stream_dataset(self, dataset_id, output_path):
        """Stream a dataset to file"""
        with requests.get(
            f"{self.base_url}/api/v1/datasets/{dataset_id}/stream",
            headers=self.headers,
            stream=True
        ) as r:
            r.raise_for_status()
            with open(output_path, 'wb') as f:
                for chunk in r.iter_content(chunk_size=8192):
                    f.write(chunk)
    
    def upload_model(self, model_name, model_path, metadata=None):
        """Upload a model with optional metadata"""
        files = {
            'model': open(model_path, 'rb')
        }
        if metadata:
            files['metadata'] = open(metadata, 'rb')
        
        response = requests.post(
            f"{self.base_url}/api/v1/models/{self.primal}/{model_name}",
            headers=self.headers,
            files=files
        )
        return response.json()
    
    def download_model(self, model_name, output_path, version="latest"):
        """Download a model"""
        with requests.get(
            f"{self.base_url}/api/v1/models/{self.primal}/{model_name}/download",
            headers=self.headers,
            params={"version": version},
            stream=True
        ) as r:
            r.raise_for_status()
            with open(output_path, 'wb') as f:
                for chunk in r.iter_content(chunk_size=8192):
                    f.write(chunk)

# Usage
client = NestGateClient(
    primal="beardog",
    api_key=os.environ["BEARDOG_API_KEY"]
)

# List datasets
datasets = client.list_datasets()
print("Available datasets:", datasets)

# Download training data
client.stream_dataset("hsm_training_v1", "training_data.tar.gz")

# Upload model
client.upload_model(
    "hsm_model",
    "hsm_v2.safetensors",
    metadata="metadata.json"
)

# Download model
client.download_model("hsm_model", "downloaded_model.safetensors")
```

---

## 🦀 **Rust Client Example**

```rust
use reqwest::Client;
use std::fs::File;
use std::io::copy;

pub struct NestGateClient {
    primal: String,
    api_key: String,
    base_url: String,
    client: Client,
}

impl NestGateClient {
    pub fn new(primal: &str, api_key: &str) -> Self {
        Self {
            primal: primal.to_string(),
            api_key: api_key.to_string(),
            base_url: "http://localhost:9000".to_string(),
            client: Client::new(),
        }
    }
    
    pub async fn list_datasets(&self) -> Result<Vec<Dataset>, Box<dyn std::error::Error>> {
        let response = self.client
            .get(&format!("{}/api/v1/datasets", self.base_url))
            .header("X-Primal", &self.primal)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?
            .json()
            .await?;
        
        Ok(response)
    }
    
    pub async fn stream_dataset(&self, dataset_id: &str, output_path: &str) 
        -> Result<(), Box<dyn std::error::Error>> 
    {
        let mut response = self.client
            .get(&format!("{}/api/v1/datasets/{}/stream", self.base_url, dataset_id))
            .header("X-Primal", &self.primal)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;
        
        let mut file = File::create(output_path)?;
        copy(&mut response.bytes().await?, &mut file)?;
        
        Ok(())
    }
    
    pub async fn upload_model(&self, model_name: &str, model_path: &str) 
        -> Result<(), Box<dyn std::error::Error>> 
    {
        let form = reqwest::multipart::Form::new()
            .file("model", model_path).await?;
        
        self.client
            .post(&format!("{}/api/v1/models/{}/{}", 
                self.base_url, self.primal, model_name))
            .header("X-Primal", &self.primal)
            .header("X-API-Key", &self.api_key)
            .multipart(form)
            .send()
            .await?;
        
        Ok(())
    }
}

// Usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = NestGateClient::new(
        "beardog",
        &std::env::var("BEARDOG_API_KEY")?
    );
    
    // List datasets
    let datasets = client.list_datasets().await?;
    println!("Datasets: {:?}", datasets);
    
    // Stream training data
    client.stream_dataset("hsm_training_v1", "training.tar.gz").await?;
    
    // Upload model
    client.upload_model("hsm_model", "hsm_v2.safetensors").await?;
    
    Ok(())
}
```

---

## 📊 **Monitoring & Metrics**

### Check Service Health

```bash
curl http://localhost:9000/health
```

### Get Service Metrics

```bash
curl http://localhost:9000/metrics
```

### View Your Quota

```bash
curl http://localhost:9000/api/v1/quota \
  -H "X-Primal: YOUR_PRIMAL" \
  -H "X-API-Key: YOUR_KEY"
```

---

## 🐛 **Troubleshooting**

### Connection Refused

```bash
# Check if service is running
curl http://localhost:9000/health

# If not, start it
cd /home/eastgate/Development/ecoPrimals/nestgate/showcase
./scripts/start_data_service.sh
```

### Authentication Failed

```bash
# Generate new API key
./scripts/generate_api_key.sh --primal YOUR_PRIMAL

# Test with new key
curl http://localhost:9000/api/v1/datasets \
  -H "X-Primal: YOUR_PRIMAL" \
  -H "X-API-Key: NEW_KEY"
```

### Slow Downloads

```bash
# Check network throughput
curl http://localhost:9000/api/v1/datasets/test_1gb/stream \
  -H "X-Primal: YOUR_PRIMAL" \
  --output test.dat \
  --progress-bar

# Should see >200 MB/s
```

### Dataset Not Found

```bash
# List all available datasets
curl http://localhost:9000/api/v1/datasets

# Generate test dataset if needed
cd showcase
./data/generators/generate_large_dataset.sh \
  --size 5GB --type mixed --primal YOUR_PRIMAL
```

---

## 📞 **Support**

### Documentation
- Full API docs: `showcase/ECOSYSTEM_INTEGRATION.md`
- Integration tests: `showcase/integration_tests/`
- Examples: `showcase/examples/`

### Testing
Run integration test for your primal:

```bash
cd showcase/integration_tests/scenarios
./beardog_hsm_integration.sh      # For beardog
./squirrel_llm_integration.sh     # For squirrel (coming soon)
./toadstool_vision_integration.sh # For toadstool (coming soon)
```

### Contact
- Issues: Create GitHub issue with tag `integration`
- Chat: #nestgate-integration channel
- Docs: `/home/eastgate/Development/ecoPrimals/nestgate/showcase/`

---

## ✅ **Quick Checklist**

Before integration, ensure:

- [ ] NestGate data service is running
- [ ] You have your primal API key
- [ ] You can access health endpoint
- [ ] You can list datasets
- [ ] You can download a test dataset
- [ ] You can upload a test model
- [ ] Performance meets your requirements

---

## 🎉 **You're Ready!**

Your primal is now connected to NestGate's data service!

**Next steps**:
1. Run your primal's integration test
2. Monitor performance metrics
3. Provide feedback on data service
4. Report any issues

**Happy integrating!** 🚀

---

*Quick Start Guide v1.0.0*  
*Last Updated: November 10, 2025*  
*NestGate Ecosystem Integration*

