// Data Analysis - Entropy detection and format recognition
//
// This module analyzes incoming data to determine optimal storage strategy

use anyhow::Result;
use bytes::Bytes;
use std::collections::HashMap;

/// Data analyzer
pub struct DataAnalyzer {
    format_detector: FormatDetector,
}

impl DataAnalyzer {
    pub fn new() -> Self {
        Self {
            format_detector: FormatDetector::new(),
        }
    }
    
    /// Analyze data characteristics
    pub async fn analyze(&self, data: &[u8]) -> Result<DataAnalysis> {
        // For very large data, sample instead of analyzing all
        let sample = if data.len() > 64 * 1024 {
            &data[..64 * 1024]  // 64KB sample
        } else {
            data
        };
        
        // Calculate entropy (Shannon information theory)
        let entropy = calculate_entropy(sample);
        
        // Detect format from magic bytes and patterns
        let format = self.format_detector.detect(data)?;
        
        // Estimate compressibility
        let compressibility = estimate_compressibility(entropy, sample);
        
        Ok(DataAnalysis {
            size: data.len(),
            entropy,
            format,
            compressibility,
            is_text: is_text_data(sample),
        })
    }
}

impl Default for DataAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of data analysis
#[derive(Debug, Clone)]
pub struct DataAnalysis {
    pub size: usize,
    pub entropy: f64,              // 0.0 to 8.0 bits/byte
    pub format: DataFormat,
    pub compressibility: f64,      // 0.0 to 1.0 (estimate)
    pub is_text: bool,
}

/// Detected data format
#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    // Already compressed
    Gzip,
    Bzip2,
    Xz,
    Zstd,
    Lz4,
    
    // Archives
    Zip,
    Tar,
    TarGz,
    
    // Images (compressed)
    Jpeg,
    Png,
    Webp,
    
    // Video/Audio (compressed)
    Mp4,
    Mkv,
    Mp3,
    
    // Genomic formats
    Fasta,
    Fastq,
    FastqGz,
    Bam,
    Vcf,
    
    // Scientific formats
    Hdf5,
    NetCdf,
    
    // Text formats
    Text,
    Json,
    Xml,
    Csv,
    
    // Binary
    Binary,
    
    // Unknown
    Unknown,
}

impl DataFormat {
    /// Is this format already compressed?
    pub fn is_compressed(&self) -> bool {
        matches!(self,
            DataFormat::Gzip | DataFormat::Bzip2 | DataFormat::Xz |
            DataFormat::Zstd | DataFormat::Lz4 | DataFormat::Zip |
            DataFormat::TarGz | DataFormat::Jpeg | DataFormat::Png |
            DataFormat::Webp | DataFormat::Mp4 | DataFormat::Mkv |
            DataFormat::Mp3 | DataFormat::FastqGz | DataFormat::Bam
        )
    }
    
    /// Should we attempt compression?
    pub fn should_compress(&self) -> bool {
        !self.is_compressed()
    }
}

/// Format detector using magic bytes
pub struct FormatDetector {
    magic_bytes: HashMap<Vec<u8>, DataFormat>,
}

impl FormatDetector {
    pub fn new() -> Self {
        let mut magic_bytes = HashMap::new();
        
        // Compressed formats
        magic_bytes.insert(vec![0x1f, 0x8b], DataFormat::Gzip);
        magic_bytes.insert(vec![0x42, 0x5a, 0x68], DataFormat::Bzip2);
        magic_bytes.insert(vec![0xfd, 0x37, 0x7a, 0x58, 0x5a, 0x00], DataFormat::Xz);
        magic_bytes.insert(vec![0x28, 0xb5, 0x2f, 0xfd], DataFormat::Zstd);
        magic_bytes.insert(vec![0x04, 0x22, 0x4d, 0x18], DataFormat::Lz4);
        
        // Archives
        magic_bytes.insert(vec![0x50, 0x4b, 0x03, 0x04], DataFormat::Zip);
        magic_bytes.insert(vec![0x50, 0x4b, 0x05, 0x06], DataFormat::Zip);
        
        // Images
        magic_bytes.insert(vec![0xff, 0xd8, 0xff], DataFormat::Jpeg);
        magic_bytes.insert(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a], DataFormat::Png);
        magic_bytes.insert(vec![0x52, 0x49, 0x46, 0x46], DataFormat::Webp); // "RIFF"
        
        // Video/Audio
        magic_bytes.insert(vec![0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70], DataFormat::Mp4);
        magic_bytes.insert(vec![0xff, 0xfb], DataFormat::Mp3);
        magic_bytes.insert(vec![0xff, 0xf3], DataFormat::Mp3);
        magic_bytes.insert(vec![0xff, 0xf2], DataFormat::Mp3);
        
        // Scientific
        magic_bytes.insert(vec![0x89, 0x48, 0x44, 0x46, 0x0d, 0x0a, 0x1a, 0x0a], DataFormat::Hdf5);
        
        Self { magic_bytes }
    }
    
    pub fn detect(&self, data: &[u8]) -> Result<DataFormat> {
        if data.is_empty() {
            return Ok(DataFormat::Unknown);
        }
        
        // Check magic bytes
        for (magic, format) in &self.magic_bytes {
            if data.starts_with(magic) {
                return Ok(format.clone());
            }
        }
        
        // Check for text markers (genomic formats)
        if data.starts_with(b">") {
            return Ok(DataFormat::Fasta);
        }
        if data.starts_with(b"@") && data.len() > 100 {
            // Likely FASTQ
            return Ok(DataFormat::Fastq);
        }
        if data.starts_with(b"##fileformat=VCF") {
            return Ok(DataFormat::Vcf);
        }
        
        // Check if it's text
        if is_text_data(data) {
            // Try to determine text format
            if data.starts_with(b"{") || data.starts_with(b"[") {
                return Ok(DataFormat::Json);
            }
            if data.starts_with(b"<?xml") || data.starts_with(b"<") {
                return Ok(DataFormat::Xml);
            }
            // Check for CSV (heuristic: contains commas and newlines)
            if data.contains(&b',') && data.contains(&b'\n') {
                return Ok(DataFormat::Csv);
            }
            return Ok(DataFormat::Text);
        }
        
        // Default: binary
        Ok(DataFormat::Binary)
    }
}

impl Default for FormatDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Shannon entropy
pub fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    // Count byte frequencies
    let mut counts = [0u32; 256];
    for &byte in data {
        counts[byte as usize] += 1;
    }
    
    // Calculate entropy
    let len = data.len() as f64;
    counts.iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / len;
            -p * p.log2()
        })
        .sum()
}

/// Estimate compressibility from entropy and other factors
pub fn estimate_compressibility(entropy: f64, data: &[u8]) -> f64 {
    // High entropy → low compressibility
    let entropy_factor = (8.0 - entropy) / 8.0;
    
    // Detect repetition
    let repetition = detect_repetition(data);
    
    // Combine factors
    (entropy_factor * 0.6 + repetition * 0.4).clamp(0.0, 1.0)
}

/// Detect repetition in data
fn detect_repetition(data: &[u8]) -> f64 {
    if data.len() < 16 {
        return 0.0;
    }
    
    // Sample sliding windows to find repeated patterns
    const WINDOW_SIZE: usize = 8;
    let max_windows = std::cmp::min(1000, data.len() - WINDOW_SIZE);
    
    let mut pattern_counts: HashMap<&[u8], usize> = HashMap::new();
    for i in 0..max_windows {
        let window = &data[i..i + WINDOW_SIZE];
        *pattern_counts.entry(window).or_insert(0) += 1;
    }
    
    // Calculate repetition score
    let repeated = pattern_counts.values().filter(|&&c| c > 1).sum::<usize>();
    (repeated as f64 / max_windows as f64).clamp(0.0, 1.0)
}

/// Check if data is text (UTF-8)
fn is_text_data(data: &[u8]) -> bool {
    // Try to decode as UTF-8
    if let Ok(s) = std::str::from_utf8(data) {
        // Check if it contains mostly printable characters
        let printable = s.chars()
            .take(1000)  // Sample first 1000 chars
            .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
            .count();
        
        printable > 900  // >90% printable
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entropy_calculation() {
        // All zeros → low entropy
        let zeros = vec![0u8; 1000];
        assert!(calculate_entropy(&zeros) < 0.1);
        
        // Random data → high entropy
        let random: Vec<u8> = (0..1000).map(|i| (i * 7919) as u8).collect();
        assert!(calculate_entropy(&random) > 7.0);
        
        // Genomic data (4 letters) → ~2 bits
        let genomic = b"ATCGATCGATCGATCGATCG".repeat(50);
        let ent = calculate_entropy(&genomic);
        assert!(ent > 1.5 && ent < 2.5);
    }
    
    #[test]
    fn test_format_detection() {
        let detector = FormatDetector::new();
        
        // Gzip
        let gzip_data = vec![0x1f, 0x8b, 0x00];
        assert_eq!(detector.detect(&gzip_data).unwrap(), DataFormat::Gzip);
        
        // PNG
        let png_data = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        assert_eq!(detector.detect(&png_data).unwrap(), DataFormat::Png);
        
        // FASTA
        let fasta_data = b">seq1\nATCGATCG";
        assert_eq!(detector.detect(fasta_data).unwrap(), DataFormat::Fasta);
    }
    
    #[test]
    fn test_format_compression_check() {
        assert!(DataFormat::Gzip.is_compressed());
        assert!(!DataFormat::Fasta.is_compressed());
        assert!(DataFormat::Jpeg.is_compressed());
        assert!(!DataFormat::Text.is_compressed());
    }
}

