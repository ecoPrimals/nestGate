#!/usr/bin/env python3
"""
Real NCBI GenBank fetching for bioinformatics pipeline
This is production-ready code that actually works with NCBI E-utilities
"""

import requests
import time
import json
import sys
from pathlib import Path
from typing import List, Dict, Optional

# NCBI E-utilities base URL
NCBI_BASE = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils"

# NCBI API Key (provided)
API_KEY = "229b5d05c1125ffdae4f675df515d3e54a09"
EMAIL = "research@nestgate.dev"  # For NCBI tracking

class NCBIFetcher:
    """Production-ready NCBI data fetcher with rate limiting and error handling."""
    
    def __init__(self, email: str):
        self.email = email
        self.last_request = 0
        self.rate_limit = 0.34  # NCBI allows ~3 requests/second
        
    def _rate_limit(self):
        """Respect NCBI's rate limiting."""
        elapsed = time.time() - self.last_request
        if elapsed < self.rate_limit:
            time.sleep(self.rate_limit - elapsed)
        self.last_request = time.time()
        
    def search_gene(self, gene_name: str, organism: str = "human") -> List[str]:
        """
        Search for gene in NCBI and return accession numbers.
        
        Args:
            gene_name: Gene symbol (e.g., "TP53", "BRCA1")
            organism: Organism name (e.g., "human", "mouse")
            
        Returns:
            List of GenBank accession IDs
        """
        self._rate_limit()
        
        # Build search query
        if organism.lower() == "human":
            organism = "Homo sapiens"
        elif organism.lower() == "mouse":
            organism = "Mus musculus"
            
        query = f"{gene_name}[Gene] AND {organism}[Organism] AND mRNA[Filter]"
        
        # Search NCBI
        url = f"{NCBI_BASE}/esearch.fcgi"
        params = {
            "db": "nucleotide",
            "term": query,
            "retmode": "json",
            "retmax": 10,
            "email": self.email,
            "api_key": API_KEY
        }
        
        response = requests.get(url, params=params)
        response.raise_for_status()
        
        data = response.json()
        return data.get("esearchresult", {}).get("idlist", [])
    
    def fetch_sequence(self, accession_id: str) -> Dict:
        """
        Fetch sequence and metadata from NCBI.
        
        Args:
            accession_id: NCBI ID (e.g., "NM_000546")
            
        Returns:
            Dictionary with sequence, metadata, and annotations
        """
        self._rate_limit()
        
        # Fetch in FASTA format
        url = f"{NCBI_BASE}/efetch.fcgi"
        params = {
            "db": "nucleotide",
            "id": accession_id,
            "rettype": "fasta",
            "retmode": "text",
            "email": self.email,
            "api_key": API_KEY
        }
        
        response = requests.get(url, params=params)
        response.raise_for_status()
        
        fasta = response.text
        
        # Also fetch metadata in GenBank format
        self._rate_limit()
        params["rettype"] = "gb"
        
        response = requests.get(url, params=params)
        metadata_text = response.text
        
        # Parse metadata
        metadata = self._parse_genbank(metadata_text)
        metadata["fasta"] = fasta
        
        return metadata
    
    def _parse_genbank(self, gb_text: str) -> Dict:
        """Parse GenBank format to extract metadata."""
        metadata = {
            "organism": "",
            "gene": "",
            "product": "",
            "chromosome": "",
            "length": 0,
            "definition": "",
            "accession": "",
            "protein_id": "",
        }
        
        for line in gb_text.split("\n"):
            if line.startswith("DEFINITION"):
                metadata["definition"] = line.replace("DEFINITION", "").strip()
            elif line.startswith("ACCESSION"):
                metadata["accession"] = line.replace("ACCESSION", "").strip()
            elif "ORGANISM" in line:
                metadata["organism"] = line.split("ORGANISM")[1].strip()
            elif "/gene=" in line:
                metadata["gene"] = line.split('"')[1]
            elif "/product=" in line:
                metadata["product"] = line.split('"')[1]
            elif "/chromosome=" in line:
                metadata["chromosome"] = line.split('"')[1]
            elif "/protein_id=" in line:
                metadata["protein_id"] = line.split('"')[1]
                
        return metadata

def main():
    """Example usage: Fetch TP53 data from NCBI."""
    
    print("🧬 NCBI GenBank Fetcher - Production Ready")
    print("=" * 50)
    print()
    
    # Initialize fetcher
    fetcher = NCBIFetcher(EMAIL)
    
    # Search for TP53 in human
    print("🔍 Searching for TP53 gene in human...")
    try:
        ids = fetcher.search_gene("TP53", "human")
        print(f"✅ Found {len(ids)} sequences")
        print(f"   IDs: {ids[:3]}...")  # Show first 3
        print()
        
        if not ids:
            print("❌ No sequences found")
            return
            
        # Fetch the first result
        ncbi_id = ids[0]
        print(f"📥 Fetching sequence {ncbi_id}...")
        data = fetcher.fetch_sequence(ncbi_id)
        
        print("✅ Sequence fetched successfully")
        print()
        print("📋 Metadata:")
        print(f"   Accession: {data['accession']}")
        print(f"   Gene: {data['gene']}")
        print(f"   Product: {data['product']}")
        print(f"   Organism: {data['organism']}")
        print(f"   Chromosome: {data['chromosome']}")
        print(f"   Protein ID: {data['protein_id']}")
        print()
        
        # Save to file
        output_dir = Path("/tmp/nestgate_bioinfo_demo/sequences")
        output_dir.mkdir(parents=True, exist_ok=True)
        
        fasta_file = output_dir / f"{data['gene']}_human.fasta"
        with open(fasta_file, 'w') as f:
            f.write(data['fasta'])
        print(f"💾 Saved to: {fasta_file}")
        
        # Save metadata
        metadata_file = output_dir.parent / "metadata" / f"{data['gene']}_metadata.json"
        metadata_file.parent.mkdir(exist_ok=True)
        with open(metadata_file, 'w') as f:
            json.dump({k: v for k, v in data.items() if k != 'fasta'}, f, indent=2)
        print(f"💾 Metadata: {metadata_file}")
        print()
        
        print("🎉 Complete! Ready for structure prediction.")
        print()
        print("Next steps:")
        print(f"  1. View sequence: cat {fasta_file}")
        print(f"  2. Run prediction: ./run_esmfold.py {fasta_file}")
        print(f"  3. Store in NestGate: nestgate put bioinfo/data/ {output_dir}")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        print()
        print("Troubleshooting:")
        print("  • Check internet connection")
        print("  • Verify EMAIL is set correctly")
        print("  • NCBI may be temporarily unavailable")
        print("  • Try again in a few minutes")
        sys.exit(1)

if __name__ == "__main__":
    main()

