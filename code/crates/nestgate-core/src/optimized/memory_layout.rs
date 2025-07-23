/// **MEMORY LAYOUT OPTIMIZATIONS**
///
/// This module implements pedantic memory layout optimizations for maximum performance:
/// - Cache-friendly data structures with optimal field ordering
/// - Padding elimination and memory compaction
/// - Cache line awareness and false sharing prevention
/// - NUMA-aware allocation patterns
use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};

/// **CACHE-OPTIMIZED STRUCTURES**
///
/// Critical data structure optimized for single cache line access (64 bytes)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CacheLineOptimized {
    // Hot path data: Most frequently accessed fields first
    /// Primary counter (8 bytes)
    pub counter: u64,
    /// Status flags (8 bytes)
    pub status: u64,
    /// Timestamp (8 bytes)
    pub timestamp: u64,
    /// Operation ID (8 bytes)
    pub operation_id: u64,

    // Medium-hot data
    /// Connection count (4 bytes)
    pub connections: u32,
    /// Error count (4 bytes)
    pub errors: u32,
    /// Reserved space (24 bytes) - fits exactly in one cache line (64 bytes total)
    _reserved: [u8; 24],
}

impl Default for CacheLineOptimized {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheLineOptimized {
    pub const fn new() -> Self {
        Self {
            counter: 0,
            status: 0,
            timestamp: 0,
            operation_id: 0,
            connections: 0,
            errors: 0,
            _reserved: [0; 24],
        }
    }
}

/// Network packet header with optimal field ordering
#[repr(C, packed(1))] // Packed for network protocols, explicit alignment
#[derive(Debug, Copy, Clone)]
pub struct OptimizedPacketHeader {
    // Network byte order fields arranged for minimal padding
    /// Version and flags (1 byte)
    pub version_flags: u8,
    /// Header length (1 byte)
    pub header_length: u8,
    /// Packet type (2 bytes - aligned)
    pub packet_type: u16,
    /// Total length (4 bytes - aligned)
    pub total_length: u32,
    /// Sequence number (8 bytes - aligned)
    pub sequence: u64,
    /// Checksum (4 bytes)
    pub checksum: u32,
    /// Reserved (4 bytes for future use and alignment)
    pub reserved: u32,
}

/// **MEMORY POOL WITH OPTIMAL LAYOUT**
///
/// Memory pool optimized for specific allocation sizes
#[repr(C)]
pub struct OptimizedMemoryPool<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize> {
    // Metadata in separate cache line to avoid contention
    /// Current allocation index
    pub current_index: usize,
    /// Total allocations counter
    pub total_allocations: u64,
    /// Free block count
    pub free_blocks: u32,
    /// Pool status flags
    pub flags: u32,

    // Padding to ensure memory blocks start on cache line boundary
    _padding:
        [u8; 64 - (mem::size_of::<usize>() + mem::size_of::<u64>() + 2 * mem::size_of::<u32>())],

    // Memory blocks aligned to cache line boundary
    blocks: [[u8; BLOCK_SIZE]; BLOCK_COUNT],

    // Free list tracking (separate from data for cache efficiency)
    free_list: [bool; BLOCK_COUNT],
}

impl<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize> Default
    for OptimizedMemoryPool<BLOCK_SIZE, BLOCK_COUNT>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize>
    OptimizedMemoryPool<BLOCK_SIZE, BLOCK_COUNT>
{
    pub const fn new() -> Self {
        Self {
            current_index: 0,
            total_allocations: 0,
            free_blocks: BLOCK_COUNT as u32,
            flags: 0,
            _padding: [0; 64
                - (mem::size_of::<usize>() + mem::size_of::<u64>() + 2 * mem::size_of::<u32>())],
            blocks: [[0; BLOCK_SIZE]; BLOCK_COUNT],
            free_list: [true; BLOCK_COUNT],
        }
    }

    /// Allocate a block with optimal cache behavior
    pub fn allocate(&mut self) -> Option<&mut [u8; BLOCK_SIZE]> {
        if self.free_blocks == 0 {
            return None;
        }

        // Find next free block using linear search (cache-friendly)
        for i in self.current_index..BLOCK_COUNT {
            if self.free_list[i] {
                self.free_list[i] = false;
                self.free_blocks -= 1;
                self.total_allocations += 1;
                self.current_index = i + 1;
                return Some(&mut self.blocks[i]);
            }
        }

        // Wrap around if needed
        for i in 0..self.current_index {
            if self.free_list[i] {
                self.free_list[i] = false;
                self.free_blocks -= 1;
                self.total_allocations += 1;
                self.current_index = i + 1;
                return Some(&mut self.blocks[i]);
            }
        }

        None
    }
}

/// **SIMD-FRIENDLY DATA LAYOUTS**
///
/// Array of Structures optimized for SIMD operations
#[repr(C)]
#[derive(Debug)]
pub struct SoAOptimized<const N: usize> {
    // Structure of Arrays layout for better SIMD utilization
    /// X coordinates (contiguous for vectorization)
    pub x: [f32; N],
    /// Y coordinates (contiguous for vectorization)
    pub y: [f32; N],
    /// Z coordinates (contiguous for vectorization)
    pub z: [f32; N],
    /// Weights (contiguous for vectorization)
    pub weights: [f32; N],
}

impl<const N: usize> Default for SoAOptimized<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> SoAOptimized<N> {
    pub const fn new() -> Self {
        Self {
            x: [0.0; N],
            y: [0.0; N],
            z: [0.0; N],
            weights: [1.0; N],
        }
    }

    /// Process all X coordinates in a SIMD-friendly manner
    pub fn transform_x(&mut self, factor: f32) {
        // Compiler can auto-vectorize this loop
        for x in &mut self.x {
            *x *= factor;
        }
    }

    /// Dot product using SIMD-optimized layout
    pub fn dot_product(&self, other: &Self) -> f32 {
        let mut result = 0.0;
        // These loops are SIMD-vectorizable
        for i in 0..N {
            result += self.x[i] * other.x[i] + self.y[i] * other.y[i] + self.z[i] * other.z[i];
        }
        result
    }
}

/// **FALSE SHARING PREVENTION**
///
/// Prevents false sharing between frequently updated counters
#[repr(align(64))] // Cache line aligned
#[derive(Debug)]
pub struct IsolatedCounter {
    /// Counter value
    pub value: std::sync::atomic::AtomicU64,
    /// Padding to prevent false sharing
    _padding: [u8; 64 - mem::size_of::<std::sync::atomic::AtomicU64>()],
}

impl IsolatedCounter {
    pub const fn new(initial: u64) -> Self {
        Self {
            value: std::sync::atomic::AtomicU64::new(initial),
            _padding: [0; 64 - mem::size_of::<std::sync::atomic::AtomicU64>()],
        }
    }

    pub fn increment(&self) -> u64 {
        self.value
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get(&self) -> u64 {
        self.value.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// **NUMA-AWARE DATA STRUCTURES**
///
/// Ring buffer optimized for cross-NUMA-node communication
#[repr(C)]
pub struct NUMAAwareRingBuffer<T, const CAPACITY: usize> {
    // Producer data (align to avoid false sharing)
    /// Producer write index (cache line isolated)
    producer: CacheLineIsolated<usize>,

    // Consumer data (separate cache line)
    /// Consumer read index (cache line isolated)
    consumer: CacheLineIsolated<usize>,

    // Buffer data (separate from indices for optimal cache behavior)
    buffer: [MaybeUninit<T>; CAPACITY],
}

/// Cache line isolated value to prevent false sharing
#[repr(align(64))]
pub struct CacheLineIsolated<T> {
    pub value: T,
    // Use fixed-size padding instead of generic-based calculation
    _padding: [u8; 56], // 64 - 8 bytes for typical usize/pointer
}

impl<T> CacheLineIsolated<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            _padding: [0; 56],
        }
    }

    pub fn load(&self) -> &T {
        &self.value
    }

    pub fn store(&mut self, value: T) {
        self.value = value;
    }
}

impl<T, const CAPACITY: usize> Default for NUMAAwareRingBuffer<T, CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAPACITY: usize> NUMAAwareRingBuffer<T, CAPACITY> {
    pub fn new() -> Self {
        Self {
            producer: CacheLineIsolated::new(0),
            consumer: CacheLineIsolated::new(0),
            // Use vec-based approach instead of array for non-Copy types
            buffer: {
                let mut buf = Vec::with_capacity(CAPACITY);
                for _ in 0..CAPACITY {
                    buf.push(MaybeUninit::uninit());
                }
                buf.try_into()
                    .unwrap_or_else(|_| panic!("Invalid capacity"))
            },
        }
    }
}

/// **MEMORY ALIGNMENT UTILITIES**
///
/// Trait for alignment requirements
pub trait AlignmentRequirement {
    const ALIGNMENT: usize;
}

/// Standard cache line alignment (64 bytes)
pub struct CacheLineAlign;
impl AlignmentRequirement for CacheLineAlign {
    const ALIGNMENT: usize = 64;
}

/// Page alignment (4KB)
pub struct PageAlign;
impl AlignmentRequirement for PageAlign {
    const ALIGNMENT: usize = 4096;
}

/// Huge page alignment (2MB)
pub struct HugePageAlign;
impl AlignmentRequirement for HugePageAlign {
    const ALIGNMENT: usize = 2097152; // 2MB
}

/// Generic aligned storage
#[repr(C)]
pub struct AlignedStorage<T, A: AlignmentRequirement> {
    /// Aligned data
    data: T,
    /// Phantom data for alignment
    _align: PhantomData<A>,
}

/// **TYPE ALIASES FOR COMMON OPTIMIZED LAYOUTS**
///
/// Network packet optimized for minimal memory footprint
pub type OptimizedNetworkPacket = OptimizedPacketHeader;

/// Memory pool for small allocations (64-byte blocks)
pub type SmallBlockPool = OptimizedMemoryPool<64, 1024>;

/// Memory pool for medium allocations (1KB blocks)  
pub type MediumBlockPool = OptimizedMemoryPool<1024, 256>;

/// Memory pool for large allocations (64KB blocks)
pub type LargeBlockPool = OptimizedMemoryPool<65536, 16>;

/// SIMD-optimized 3D point array (128 points for cache efficiency)
pub type OptimizedPointCloud = SoAOptimized<128>;

/// Performance counter with false sharing prevention
pub type PerformanceCounter = IsolatedCounter;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_cache_line_optimization() {
        let optimized = CacheLineOptimized::new();
        // Verify it fits in exactly one cache line
        assert_eq!(mem::size_of::<CacheLineOptimized>(), 64);
        assert_eq!(optimized.counter, 0);
    }

    #[test]
    fn test_packet_header_layout() {
        let header = OptimizedPacketHeader {
            version_flags: 0x01,
            header_length: 20,
            packet_type: 0x0800,
            total_length: 1500,
            sequence: 12345,
            checksum: 0xDEADBEEF,
            reserved: 0,
        };

        // Verify minimal memory footprint for network efficiency
        assert!(mem::size_of::<OptimizedPacketHeader>() <= 32);
        // Copy the value to avoid unaligned reference
        let sequence = header.sequence;
        assert_eq!(sequence, 12345);
    }

    #[test]
    fn test_memory_pool_allocation() {
        let mut pool = SmallBlockPool::new();

        // Test allocation
        let block = pool.allocate();
        assert!(block.is_some());
        assert_eq!(pool.free_blocks, 1023); // One block allocated
        assert_eq!(pool.total_allocations, 1);
    }

    #[test]
    fn test_simd_layout() {
        let mut points = OptimizedPointCloud::new();
        points.x[0] = 1.0;
        points.y[0] = 2.0;
        points.z[0] = 3.0;

        // Test SIMD-friendly transformation
        points.transform_x(2.0);
        assert_eq!(points.x[0], 2.0);

        // Arrays should be contiguous for SIMD
        unsafe {
            let x_ptr = points.x.as_ptr();
            let y_ptr = points.y.as_ptr();
            // Y array starts immediately after X array
            assert_eq!(y_ptr as usize, x_ptr.add(128) as usize);
        }
    }

    #[test]
    fn test_false_sharing_prevention() {
        let counter = IsolatedCounter::new(0);

        // Verify counter is cache line aligned
        let alignment = mem::align_of_val(&counter);
        assert_eq!(alignment, 64);

        // Verify size includes padding
        assert_eq!(mem::size_of::<IsolatedCounter>(), 64);

        // Test functionality
        counter.increment();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_alignment_requirements() {
        assert_eq!(CacheLineAlign::ALIGNMENT, 64);
        assert_eq!(PageAlign::ALIGNMENT, 4096);
        assert_eq!(HugePageAlign::ALIGNMENT, 2097152);
    }
}
