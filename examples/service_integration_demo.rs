// Service Integration Demo - Shows the new AdaptiveStorageService in action
//
// This demonstrates how the legacy StorageManagerService can gradually
// adopt the new NestGateStorage engine through the integration layer.
//
// Run with: cargo run --example service_integration_demo

use anyhow::Result;
use nestgate_core::services::storage::service_integration::AdaptiveStorageService;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║    🔗 Service Integration Demo                               ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    
    // Create service
    let temp_dir = TempDir::new()?;
    let service = AdaptiveStorageService::new(temp_dir.path().to_path_buf());
    service.initialize().await?;
    
    println!("✅ Service initialized");
    println!();
    
    // Example 1: Analyze data before storing
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Analyze Data");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let genomic_data = b"ATCGATCGATCG".repeat(100);
    let analysis = service.analyze_data(&genomic_data).await?;
    
    println!("Data size:              {} bytes", analysis.size);
    println!("Entropy:                {:.2} bits/byte", analysis.entropy);
    println!("Format:                 {}", analysis.format);
    println!("Compressibility:        {:.1}%", analysis.compressibility_estimate * 100.0);
    println!("Is text:                {}", analysis.is_text);
    println!("Recommended strategy:   {}", analysis.recommended_strategy);
    println!();
    
    // Example 2: Store genomic data
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Store Genomic Data");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let receipt = service.store_data(genomic_data.to_vec()).await?;
    
    println!("Hash:                   {}", receipt.hash);
    println!("Original size:          {} bytes", receipt.original_size);
    println!("Stored size:            {} bytes", receipt.stored_size);
    println!("Compression ratio:      {:.2}:1", receipt.compression_ratio);
    println!("Strategy used:          {}", receipt.strategy);
    println!("Encryption:             {}", receipt.encryption);
    println!();
    
    // Example 3: Store random data (should use passthrough)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Store Random Data");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let random_data = create_random_data(10_000);
    let receipt = service.store_data(random_data.clone()).await?;
    
    println!("Hash:                   {}", receipt.hash);
    println!("Original size:          {} bytes", receipt.original_size);
    println!("Stored size:            {} bytes", receipt.stored_size);
    println!("Strategy used:          {}", receipt.strategy);
    
    if receipt.strategy.contains("Raw") {
        println!("✅ Correctly detected high entropy - used passthrough!");
    }
    println!();
    
    // Example 4: Retrieve data
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 4: Retrieve Data");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let retrieved = service.retrieve_data(&receipt.hash).await?;
    println!("Retrieved:              {} bytes", retrieved.len());
    
    if retrieved == random_data {
        println!("✅ Data integrity verified!");
    } else {
        println!("❌ Data mismatch!");
    }
    println!();
    
    // Example 5: Check existence
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 5: Check Existence");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let exists = service.data_exists(&receipt.hash).await?;
    println!("Data exists:            {}", exists);
    
    let fake_hash = "0".repeat(64);
    let not_exists = service.data_exists(&fake_hash).await?;
    println!("Fake data exists:       {}", not_exists);
    println!();
    
    // Example 6: Get metrics
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 6: Service Metrics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let metrics = service.get_metrics();
    
    println!("Total operations:       {}", metrics.total_operations);
    println!("Total bytes stored:     {} bytes", metrics.total_bytes_stored);
    println!("Total bytes saved:      {} bytes", metrics.total_bytes_saved);
    println!("Savings percentage:     {:.1}%", metrics.savings_percent);
    println!("Avg compression ratio:  {:.2}:1", metrics.compression_ratio_avg);
    println!("Avg entropy:            {:.2} bits/byte", metrics.entropy_avg);
    println!("Avg operation time:     {:.2} ms", metrics.operation_time_avg_ms);
    println!();
    
    println!("Strategy breakdown:");
    for (strategy, count) in metrics.strategy_counts.iter() {
        println!("  {}: {}", strategy, count);
    }
    println!();
    
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║    ✅ Integration Demo Complete!                             ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    
    Ok(())
}

fn create_random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut data = vec![0u8; size];
    rng.fill_bytes(&mut data);
    data
}

