use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let ops_per_sec = 100u64;
    let duration = 20u64;
    
    let sleep_duration = Duration::from_micros(1_000_000 / ops_per_sec.max(1));
    let total_operations = ops_per_sec * duration;
    let end_time = Instant::now() + Duration::from_secs(duration);
    
    println!("ops_per_sec: {}", ops_per_sec);
    println!("duration: {} seconds", duration);
    println!("sleep_duration: {:?}", sleep_duration);
    println!("total_operations: {}", total_operations);
    println!("end_time: {:?}", end_time);
    
    let start = Instant::now();
    let mut op_count = 0u64;
    
    while Instant::now() < end_time && op_count < total_operations {
        op_count += 1;
        if op_count % 100 == 0 {
            println!("op_count: {}, elapsed: {:?}", op_count, start.elapsed());
        }
        if op_count > 10 {
            break; // Don't run forever
        }
    }
    
    println!("Final op_count: {}, elapsed: {:?}", op_count, start.elapsed());
    println!("Condition: now < end_time: {}", Instant::now() < end_time);
    println!("Condition: op_count < total_operations: {}", op_count < total_operations);
}
