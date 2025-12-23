# 🧬 **BIOINFORMATICS PIPELINE: NCBI → PROTEIN PREDICTION**

**Practical Implementation Guide Using Your Hardware**

---

## 🎯 **PIPELINE OVERVIEW**

```
Paper → NCBI IDs → Download → Store → Predict → Analyze
   │        │          │         │        │         │
   AI      API     NestGate  Westgate Northgate  Results
```

**Real-World Use Case**: You're reading a genomics paper that references 50 protein sequences. You want to:
1. Extract NCBI identifiers from the paper (AI-assisted)
2. Download sequences and metadata from NCBI
3. Store everything with full provenance in NestGate
4. Run ESMFold/OpenFold predictions (GPU-accelerated)
5. Analyze results and generate reports

---

## 🏗️ **ARCHITECTURE**

### **Hardware Mapping**

```
┌────────────────────────────────────────────────────────┐
│              PROTEIN PREDICTION PIPELINE               │
├────────────────────────────────────────────────────────┤
│                                                        │
│  📄 EASTGATE (Development)                            │
│     - Paper analysis (AI extracts IDs)                │
│     - Pipeline orchestration                           │
│     - Results visualization                            │
│     Role: Control plane                                │
│                                                         │
│  🗄️ WESTGATE (Storage - 86TB HDD)                     │
│     - Raw NCBI data (compressed)                       │
│     - Metadata and papers                              │
│     - Long-term archive                                │
│     Role: NestGate cold storage                        │
│                                                         │
│  ⚡ STRANDGATE (Parallel - 64 cores, 56TB)            │
│     - BLAST searches (CPU-intensive)                   │
│     - Batch job coordination                           │
│     - Data preprocessing                               │
│     Role: Parallel compute + NestGate warm tier        │
│                                                         │
│  🚀 NORTHGATE (AI - RTX 5090, 192GB RAM, 5TB NVMe)    │
│     - ESMFold inference (GPU)                          │
│     - OpenFold predictions                             │
│     - Model loading from cache                         │
│     Role: GPU compute + NestGate hot tier              │
│                                                         │
└────────────────────────────────────────────────────────┘
```

### **Data Flow**

```
1. Paper (PDF) → Eastgate → Extract IDs → NCBI API
                     │
2. NCBI Data → Westgate NestGate (cold storage)
                     │
3. Access Trigger → Auto-tier to Strandgate (warm)
                     │
4. Prediction Request → Prefetch to Northgate (hot)
                     │
5. ESMFold (GPU) → Results → Back to Westgate (archive)
```

---

## 📋 **PREREQUISITES**

### **Software**

```bash
# On all nodes
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/your-org/nestgate.git
cd nestgate && cargo build --release

# On Northgate (AI node)
pip install esmfold-inference biopython torch

# On Eastgate (dev node)
pip install biopython requests pandas matplotlib
```

### **NestGate Configuration**

```bash
# WESTGATE - Cold storage
cat > /etc/nestgate/westgate.toml << 'EOF'
[node]
name = "westgate"
role = "storage"

[storage]
pool = "biodata"
path = "/data/biodata"
tier = "cold"
compression = "lz4"
dedup = true

[storage.policies]
auto_tier_up = true
tier_up_threshold = "3_accesses_per_day"

[exports]
nfs = ["192.168.1.0/24"]
smb = true
s3_compatible = true

[api]
bind = "0.0.0.0:8080"
enable_metrics = true
EOF

# STRANDGATE - Warm tier
cat > /etc/nestgate/strandgate.toml << 'EOF'
[node]
name = "strandgate"
role = "smart-tier"

[storage]
hot_device = "/dev/nvme0n1"
warm_device = "/dev/sda"
cold_device = "/dev/sdb"
tier = "warm"

[storage.policies]
cache_size = "20TB"
eviction = "lru"
prefetch = true

[upstream]
cold_tier = "westgate:8080"
EOF

# NORTHGATE - Hot tier + GPU
cat > /etc/nestgate/northgate.toml << 'EOF'
[node]
name = "northgate"
role = "hot-cache"

[storage]
device = "/dev/nvme0n1"
capacity = "4TB"
tier = "hot"

[storage.policies]
gpu_direct = true
pin_memory = true

[upstream]
warm_tier = "strandgate:8080"
cold_tier = "westgate:8080"

[gpu]
devices = [0]  # RTX 5090
memory_fraction = 0.8
EOF
```

---

## 🚀 **IMPLEMENTATION**

### **Step 1: Paper Analysis (AI-Assisted)**

```python
#!/usr/bin/env python3
"""
extract_ncbi_ids.py
Extract NCBI protein IDs from research papers using AI
"""

import re
from pathlib import Path
from typing import List, Dict
import openai  # or local LLM

def extract_protein_ids_from_paper(pdf_path: str) -> List[Dict]:
    """
    Extract NCBI protein IDs from a paper
    Uses AI to understand context and extract relevant IDs
    """
    
    # Read paper (you'd use PyPDF2 or similar)
    text = read_pdf(pdf_path)
    
    # Use AI to extract structured information
    prompt = f"""
    Extract all NCBI protein identifiers from this research paper.
    Return as JSON with: accession, description, context.
    
    Paper text:
    {text[:4000]}  # First 4000 chars
    """
    
    response = openai.ChatCompletion.create(
        model="gpt-4",
        messages=[{"role": "user", "content": prompt}]
    )
    
    # Parse AI response
    import json
    ids = json.loads(response.choices[0].message.content)
    
    # Validate format (NP_XXXXXX, XP_XXXXXX, etc.)
    validated = []
    for item in ids:
        accession = item['accession']
        if re.match(r'^[NX]P_\d+(\.\d+)?$', accession):
            validated.append(item)
    
    return validated

# Usage
paper = "/path/to/genomics_paper.pdf"
protein_ids = extract_protein_ids_from_paper(paper)

print(f"Found {len(protein_ids)} protein IDs:")
for p in protein_ids:
    print(f"  - {p['accession']}: {p['description']}")
```

### **Step 2: NCBI Download + NestGate Storage**

```python
#!/usr/bin/env python3
"""
fetch_and_store.py
Download NCBI data and store in NestGate with full provenance
"""

from Bio import Entrez, SeqIO
import nestgate
from datetime import datetime
import hashlib

# Configure NCBI
Entrez.email = "your.email@institution.edu"  # Required by NCBI
Entrez.api_key = "your_ncbi_api_key"  # 10 req/sec with key

# Configure NestGate
storage = nestgate.Client("westgate:8080")

def fetch_protein_from_ncbi(accession: str) -> Dict:
    """Fetch protein data from NCBI"""
    
    # Fetch sequence
    handle = Entrez.efetch(
        db="protein",
        id=accession,
        rettype="fasta",
        retmode="text"
    )
    sequence = SeqIO.read(handle, "fasta")
    handle.close()
    
    # Fetch metadata
    handle = Entrez.efetch(
        db="protein",
        id=accession,
        rettype="gb",
        retmode="xml"
    )
    metadata = Entrez.read(handle)
    handle.close()
    
    return {
        "accession": accession,
        "sequence": str(sequence.seq),
        "description": sequence.description,
        "length": len(sequence.seq),
        "metadata": metadata,
        "source_url": f"https://www.ncbi.nlm.nih.gov/protein/{accession}",
        "download_timestamp": datetime.utcnow().isoformat(),
        "checksum": hashlib.sha256(str(sequence.seq).encode()).hexdigest()
    }

def store_in_nestgate(protein_data: Dict, paper_reference: str):
    """Store protein data in NestGate with full provenance"""
    
    accession = protein_data['accession']
    
    # Store FASTA sequence
    fasta_path = f"/biodata/sequences/{accession}.fasta"
    fasta_content = f">{accession} {protein_data['description']}\n{protein_data['sequence']}\n"
    
    storage.write(
        path=fasta_path,
        data=fasta_content.encode(),
        tier="cold",  # Start in cold storage
        metadata={
            "source": "ncbi",
            "accession": accession,
            "length": protein_data['length'],
            "checksum": protein_data['checksum'],
            "source_url": protein_data['source_url'],
            "download_timestamp": protein_data['download_timestamp'],
            "paper_reference": paper_reference,
            "data_type": "protein_sequence",
            "format": "fasta"
        }
    )
    
    # Store full metadata as JSON
    metadata_path = f"/biodata/metadata/{accession}.json"
    import json
    storage.write(
        path=metadata_path,
        data=json.dumps(protein_data['metadata'], indent=2).encode(),
        tier="cold",
        metadata={
            "associated_sequence": fasta_path,
            "accession": accession
        }
    )
    
    print(f"✓ Stored {accession} in NestGate (cold tier, compressed)")
    return fasta_path

# Main pipeline
def download_pipeline(protein_ids: List[Dict], paper_ref: str):
    """Download all proteins and store with provenance"""
    
    results = []
    
    for protein in protein_ids:
        accession = protein['accession']
        print(f"Fetching {accession} from NCBI...")
        
        try:
            # Download from NCBI
            data = fetch_protein_from_ncbi(accession)
            
            # Store in NestGate
            path = store_in_nestgate(data, paper_ref)
            
            results.append({
                "accession": accession,
                "status": "success",
                "path": path,
                "size": len(data['sequence'])
            })
            
        except Exception as e:
            print(f"✗ Error with {accession}: {e}")
            results.append({
                "accession": accession,
                "status": "error",
                "error": str(e)
            })
    
    return results

# Usage
paper_ref = "Smith et al. 2025, Nature Genetics"
results = download_pipeline(protein_ids, paper_ref)

# Summary
print(f"\n{'='*60}")
print(f"Download Summary:")
print(f"  Total:    {len(results)}")
print(f"  Success:  {sum(1 for r in results if r['status'] == 'success')}")
print(f"  Failed:   {sum(1 for r in results if r['status'] == 'error')}")
print(f"  Storage:  Westgate (cold tier, auto-tiering enabled)")
print(f"{'='*60}")
```

### **Step 3: Protein Prediction (GPU-Accelerated)**

```python
#!/usr/bin/env python3
"""
predict_structures.py
Run ESMFold predictions on GPU with NestGate integration
Run this on NORTHGATE (RTX 5090)
"""

import torch
import nestgate
from esmfold.v1 import ESMFold
from pathlib import Path
from typing import List

# Initialize
device = torch.device("cuda:0")  # RTX 5090
storage = nestgate.Client("localhost:8080")  # Northgate local cache

# Load ESMFold model
print("Loading ESMFold model...")
model = ESMFold.from_pretrained("facebook/esmfold_v1")
model = model.eval().to(device)
print(f"✓ Model loaded on {torch.cuda.get_device_name(0)}")

def predict_structure(accession: str) -> Dict:
    """
    Predict protein structure using ESMFold
    NestGate automatically tiers data to local GPU cache
    """
    
    # Read sequence - NestGate handles tiering automatically
    # If not in hot cache (Northgate), fetches from warm (Strandgate) or cold (Westgate)
    sequence_path = f"/biodata/sequences/{accession}.fasta"
    sequence_data = storage.read(sequence_path).decode()
    
    # Parse FASTA
    lines = sequence_data.strip().split('\n')
    sequence = ''.join(lines[1:])  # Skip header
    
    print(f"Predicting structure for {accession} ({len(sequence)} amino acids)...")
    
    # Run prediction on GPU
    with torch.no_grad():
        output = model.infer(sequence, num_recycles=4)
    
    # Extract results
    pdb_string = model.output_to_pdb(output)[0]
    confidence = output["plddt"].mean().item()
    
    # Store results back to NestGate
    result_path = f"/biodata/predictions/{accession}.pdb"
    storage.write(
        path=result_path,
        data=pdb_string.encode(),
        tier="warm",  # Predictions are warm tier
        metadata={
            "accession": accession,
            "model": "esmfold_v1",
            "confidence": confidence,
            "sequence_length": len(sequence),
            "gpu_device": "RTX 5090",
            "node": "northgate",
            "num_recycles": 4
        }
    )
    
    print(f"✓ {accession}: confidence={confidence:.2f}, saved to {result_path}")
    
    return {
        "accession": accession,
        "pdb_path": result_path,
        "confidence": confidence,
        "length": len(sequence)
    }

def batch_predict(accessions: List[str], batch_size: int = 8):
    """Batch prediction for efficiency"""
    
    results = []
    
    # Process in batches (GPU memory permitting)
    for i in range(0, len(accessions), batch_size):
        batch = accessions[i:i+batch_size]
        print(f"\nProcessing batch {i//batch_size + 1} ({len(batch)} proteins)...")
        
        for accession in batch:
            try:
                result = predict_structure(accession)
                results.append(result)
            except Exception as e:
                print(f"✗ Error with {accession}: {e}")
                results.append({
                    "accession": accession,
                    "status": "error",
                    "error": str(e)
                })
    
    return results

# Main prediction pipeline
if __name__ == "__main__":
    import sys
    
    # Read accessions from command line or file
    if len(sys.argv) > 1:
        accessions_file = sys.argv[1]
        with open(accessions_file) as f:
            accessions = [line.strip() for line in f]
    else:
        # Example
        accessions = [
            "NP_001234567",
            "NP_002345678",
            "NP_003456789"
        ]
    
    print(f"Starting predictions for {len(accessions)} proteins...")
    print(f"GPU: {torch.cuda.get_device_name(0)}")
    print(f"VRAM: {torch.cuda.get_device_properties(0).total_memory / 1e9:.1f} GB")
    
    results = batch_predict(accessions)
    
    # Summary
    print(f"\n{'='*60}")
    print(f"Prediction Summary:")
    print(f"  Total:       {len(results)}")
    print(f"  Success:     {sum(1 for r in results if 'confidence' in r)}")
    print(f"  Failed:      {sum(1 for r in results if 'error' in r)}")
    print(f"  Avg Confidence: {sum(r.get('confidence', 0) for r in results) / len(results):.2f}")
    print(f"  Storage:     NestGate (warm tier, accessible from all nodes)")
    print(f"{'='*60}")
```

### **Step 4: Analysis & Visualization**

```python
#!/usr/bin/env python3
"""
analyze_results.py
Analyze prediction results and generate reports
Run this on EASTGATE (development workstation)
"""

import nestgate
import pandas as pd
import matplotlib.pyplot as plt
from Bio.PDB import PDBParser
import json

storage = nestgate.Client("westgate:8080")

def analyze_predictions(accessions: List[str]) -> pd.DataFrame:
    """Analyze all predictions and compile results"""
    
    results = []
    
    for accession in accessions:
        # Read metadata from NestGate
        metadata_path = f"/biodata/metadata/{accession}.json"
        pred_path = f"/biodata/predictions/{accession}.pdb"
        
        try:
            # Get prediction metadata
            meta = storage.get_metadata(pred_path)
            
            results.append({
                "accession": accession,
                "length": meta['sequence_length'],
                "confidence": meta['confidence'],
                "model": meta['model'],
                "gpu_device": meta['gpu_device'],
                "prediction_path": pred_path
            })
        except Exception as e:
            print(f"Warning: Could not analyze {accession}: {e}")
    
    return pd.DataFrame(results)

def generate_report(df: pd.DataFrame, paper_ref: str):
    """Generate comprehensive analysis report"""
    
    # Statistics
    print(f"\n{'='*60}")
    print(f"Protein Prediction Analysis Report")
    print(f"Paper: {paper_ref}")
    print(f"{'='*60}\n")
    
    print(f"Dataset Statistics:")
    print(f"  Total Proteins:        {len(df)}")
    print(f"  Avg Length:            {df['length'].mean():.0f} amino acids")
    print(f"  Avg Confidence:        {df['confidence'].mean():.2f}")
    print(f"  High Confidence (>0.8): {(df['confidence'] > 0.8).sum()}")
    print(f"  Low Confidence (<0.5):  {(df['confidence'] < 0.5).sum()}")
    
    # Visualizations
    fig, axes = plt.subplots(2, 2, figsize=(12, 10))
    
    # Confidence distribution
    axes[0, 0].hist(df['confidence'], bins=20, edgecolor='black')
    axes[0, 0].set_xlabel('Confidence Score')
    axes[0, 0].set_ylabel('Count')
    axes[0, 0].set_title('Prediction Confidence Distribution')
    
    # Length vs Confidence
    axes[0, 1].scatter(df['length'], df['confidence'], alpha=0.6)
    axes[0, 1].set_xlabel('Sequence Length')
    axes[0, 1].set_ylabel('Confidence Score')
    axes[0, 1].set_title('Length vs Confidence')
    
    # Confidence by length bins
    df['length_bin'] = pd.cut(df['length'], bins=5)
    confidence_by_length = df.groupby('length_bin')['confidence'].mean()
    axes[1, 0].bar(range(len(confidence_by_length)), confidence_by_length.values)
    axes[1, 0].set_xlabel('Length Bin')
    axes[1, 0].set_ylabel('Avg Confidence')
    axes[1, 0].set_title('Confidence by Length Range')
    
    # Summary table
    axes[1, 1].axis('tight')
    axes[1, 1].axis('off')
    table_data = [
        ['Metric', 'Value'],
        ['Total Proteins', f"{len(df)}"],
        ['Avg Confidence', f"{df['confidence'].mean():.3f}"],
        ['Min/Max Length', f"{df['length'].min()}/{df['length'].max()}"],
        ['High Confidence', f"{(df['confidence'] > 0.8).sum()} ({(df['confidence'] > 0.8).sum()/len(df)*100:.1f}%)"]
    ]
    axes[1, 1].table(cellText=table_data, loc='center', cellLoc='left')
    axes[1, 1].set_title('Summary Statistics')
    
    plt.tight_layout()
    
    # Save to NestGate
    report_path = f"/biodata/reports/analysis_{datetime.now().strftime('%Y%m%d_%H%M%S')}.png"
    plt.savefig('/tmp/report.png')
    
    with open('/tmp/report.png', 'rb') as f:
        storage.write(
            path=report_path,
            data=f.read(),
            metadata={"paper_reference": paper_ref, "type": "analysis_report"}
        )
    
    print(f"\n✓ Report saved to: {report_path}")
    plt.show()

# Usage
df = analyze_predictions(accessions)
generate_report(df, "Smith et al. 2025")
```

---

## 🎬 **COMPLETE WORKFLOW SCRIPT**

```bash
#!/bin/bash
# complete_pipeline.sh
# End-to-end protein prediction pipeline

set -e  # Exit on error

PAPER_PATH="$1"
OUTPUT_DIR="./pipeline_output"

echo "🧬 Protein Prediction Pipeline"
echo "Paper: $PAPER_PATH"
echo "======================================"

# Step 1: Extract IDs from paper
echo "📄 Step 1: Extracting protein IDs from paper..."
python extract_ncbi_ids.py "$PAPER_PATH" > "$OUTPUT_DIR/protein_ids.json"
IDS_COUNT=$(jq length "$OUTPUT_DIR/protein_ids.json")
echo "✓ Found $IDS_COUNT protein IDs"

# Step 2: Download from NCBI and store in NestGate
echo "⬇️  Step 2: Downloading from NCBI..."
python fetch_and_store.py \
    --ids "$OUTPUT_DIR/protein_ids.json" \
    --paper-ref "$(basename $PAPER_PATH)" \
    --storage westgate:8080
echo "✓ Data stored in NestGate (Westgate cold tier)"

# Step 3: Trigger predictions on Northgate
echo "🚀 Step 3: Running predictions on Northgate..."
jq -r '.[].accession' "$OUTPUT_DIR/protein_ids.json" > "$OUTPUT_DIR/accessions.txt"
ssh northgate "cd ~/nestgate && python predict_structures.py $OUTPUT_DIR/accessions.txt"
echo "✓ Predictions complete (GPU: RTX 5090)"

# Step 4: Analyze results
echo "📊 Step 4: Analyzing results..."
python analyze_results.py \
    --ids "$OUTPUT_DIR/protein_ids.json" \
    --storage westgate:8080 \
    --output "$OUTPUT_DIR/report.html"
echo "✓ Report generated"

# Summary
echo ""
echo "======================================"
echo "Pipeline Complete! 🎉"
echo "======================================"
echo "Proteins processed: $IDS_COUNT"
echo "Storage: NestGate multi-tier"
echo "  - Cold (Westgate): Raw data"
echo "  - Warm (Strandgate): Cached"
echo "  - Hot (Northgate): Active predictions"
echo "Report: $OUTPUT_DIR/report.html"
echo ""
echo "View in NestGate:"
echo "  firefox http://westgate:8080/datasets/biodata"
```

---

## 📊 **EXPECTED PERFORMANCE**

### **Your Hardware Capabilities**

| Operation | Node | Time | Notes |
|-----------|------|------|-------|
| **NCBI Download** | Any | ~5 sec/protein | API rate limit |
| **Storage (NestGate)** | Westgate | ~1 sec/protein | HDD write |
| **Tier Up (Cold→Warm)** | Westgate→Strandgate | ~10 sec/GB | 1 Gbps network |
| **Tier Up (Warm→Hot)** | Strandgate→Northgate | ~10 sec/GB | 1 Gbps network |
| **ESMFold Prediction** | Northgate | ~30-60 sec/protein | RTX 5090 |
| **Batch (8 proteins)** | Northgate | ~4-6 min/batch | GPU memory |

### **Full Pipeline Timing**

```
50 Proteins Pipeline:
  Extract IDs:        ~5 minutes (AI)
  NCBI Download:      ~5 minutes (250 proteins total)
  NestGate Storage:   ~1 minute
  Prediction:         ~25-50 minutes (RTX 5090)
  Analysis:           ~2 minutes
  
  Total:             ~40-65 minutes

With 10 GbE:        ~30-45 minutes (faster tiering)
```

---

## 🔧 **TROUBLESHOOTING**

### **Common Issues**

**Issue**: "NCBI API rate limit exceeded"
```bash
# Solution: Use API key (10 req/sec vs 3 req/sec)
export NCBI_API_KEY="your_key_here"
```

**Issue**: "GPU out of memory"
```bash
# Solution: Reduce batch size or sequence length limit
python predict_structures.py --max-length 1000 --batch-size 4
```

**Issue**: "NestGate: tier migration slow"
```bash
# Solution: Manually prefetch to hot tier
nestgate prefetch \
    --from /biodata/sequences/*.fasta \
    --to-node northgate \
    --tier hot
```

---

## 🎯 **NEXT STEPS**

1. **This Week**: Test with 5-10 proteins
2. **Next Week**: Run full 50-protein pipeline
3. **Month 2**: Automate with cron/scheduled jobs
4. **Production**: Scale to 1000s of proteins

**Your hardware is perfect for bioinformatics!** 🚀

- Westgate: Unlimited storage (86TB)
- Strandgate: Parallel processing (64 cores)
- Northgate: Fast inference (RTX 5090)
- NestGate: Smart data management

**Start small, scale big!** ✨

