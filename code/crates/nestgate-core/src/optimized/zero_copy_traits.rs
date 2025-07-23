/// **PEDANTIC ZERO-COPY TRAITS**
///
/// Advanced zero-copy patterns that eliminate unnecessary allocations through
/// compile-time guarantees and lifetime-aware programming.
///
/// **Design Principles:**
/// - Lifetime-parametrized traits for compile-time safety
/// - Const generics for buffer size optimization  
/// - Associated types for zero-cost abstractions
/// - RAII patterns for automatic resource management
use std::borrow::Cow;
use std::mem::MaybeUninit;

/// **ZERO-COPY STRING OPERATIONS**
///
/// Trait for string operations that avoid unnecessary allocations
pub trait ZeroCopyString<'a> {
    type Output;

    /// Create from static string (zero allocation)
    fn from_static(s: &'static str) -> Self::Output;

    /// Create from borrowed string (zero allocation when possible)
    fn from_borrowed(s: &'a str) -> Self::Output;

    /// Create from owned string (single allocation)
    fn from_owned(s: String) -> Self::Output;

    /// Transform without allocation when possible
    fn transform<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(&str) -> Cow<'a, str>;
}

impl<'a> ZeroCopyString<'a> for Cow<'a, str> {
    type Output = Cow<'a, str>;

    fn from_static(s: &'static str) -> Self::Output {
        Cow::Borrowed(s)
    }

    fn from_borrowed(s: &'a str) -> Self::Output {
        Cow::Borrowed(s)
    }

    fn from_owned(s: String) -> Self::Output {
        Cow::Owned(s)
    }

    fn transform<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(&str) -> Cow<'a, str>,
    {
        f(&self)
    }
}

/// **COMPILE-TIME BUFFER MANAGEMENT**
///
/// Fixed-size buffer with compile-time size guarantees
#[repr(align(64))] // Cache-line aligned for optimal performance
pub struct AlignedBuffer<const N: usize> {
    data: [MaybeUninit<u8>; N],
    initialized: usize,
}

impl<const N: usize> Default for AlignedBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> AlignedBuffer<N> {
    /// Create new uninitialized buffer
    pub const fn new() -> Self {
        Self {
            data: [MaybeUninit::uninit(); N],
            initialized: 0,
        }
    }

    /// Initialize buffer with data (zero-copy when data fits)
    pub fn init_from_slice(&mut self, data: &[u8]) -> Result<&[u8], BufferError> {
        if data.len() > N {
            return Err(BufferError::BufferTooSmall {
                required: data.len(),
                available: N,
            });
        }

        // Safe because we verified the length
        for (i, &byte) in data.iter().enumerate() {
            self.data[i] = MaybeUninit::new(byte);
        }

        self.initialized = data.len();

        // Safe because we just initialized these bytes
        Ok(
            unsafe {
                std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.initialized)
            },
        )
    }

    /// Get initialized portion of buffer
    pub fn as_slice(&self) -> &[u8] {
        if self.initialized == 0 {
            return &[];
        }

        // Safe because initialized bytes are guaranteed to be valid
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.initialized) }
    }

    /// Get mutable slice for writing
    pub fn as_mut_slice(&mut self) -> &mut [MaybeUninit<u8>] {
        &mut self.data[self.initialized..]
    }

    /// Mark additional bytes as initialized
    ///
    /// # Safety
    /// Caller must ensure that `count` bytes starting from `initialized` have been written
    pub unsafe fn advance(&mut self, count: usize) {
        debug_assert!(self.initialized + count <= N);
        self.initialized += count;
    }

    /// Reset buffer to uninitialized state
    pub fn reset(&mut self) {
        self.initialized = 0;
    }

    /// Buffer capacity (compile-time constant)
    pub const fn capacity() -> usize {
        N
    }

    /// Current initialized length
    pub fn len(&self) -> usize {
        self.initialized
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.initialized == 0
    }
}

/// Buffer operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferError {
    BufferTooSmall {
        required: usize,
        available: usize,
    },
    UnalignedAccess,
    InvalidRange {
        start: usize,
        end: usize,
        len: usize,
    },
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferError::BufferTooSmall {
                required,
                available,
            } => {
                write!(
                    f,
                    "Buffer too small: need {required} bytes, have {available}"
                )
            }
            BufferError::UnalignedAccess => {
                write!(f, "Unaligned memory access attempted")
            }
            BufferError::InvalidRange { start, end, len } => {
                write!(
                    f,
                    "Invalid range [{start}..{end}) for buffer of length {len}"
                )
            }
        }
    }
}

impl std::error::Error for BufferError {}

/// **STACK-ALLOCATED STRING BUILDER**
///
/// Build strings on the stack without heap allocation for small strings
pub struct StackStringBuilder<const N: usize> {
    buffer: AlignedBuffer<N>,
}

impl<const N: usize> Default for StackStringBuilder<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> StackStringBuilder<N> {
    pub const fn new() -> Self {
        Self {
            buffer: AlignedBuffer::new(),
        }
    }

    /// Append string slice (zero allocation)
    pub fn push_str(&mut self, s: &str) -> Result<(), BufferError> {
        let bytes = s.as_bytes();
        if self.buffer.len() + bytes.len() > N {
            return Err(BufferError::BufferTooSmall {
                required: self.buffer.len() + bytes.len(),
                available: N,
            });
        }

        let slice = self.buffer.as_mut_slice();

        for (i, &byte) in bytes.iter().enumerate() {
            slice[i] = MaybeUninit::new(byte);
        }

        unsafe {
            self.buffer.advance(bytes.len());
        }

        Ok(())
    }

    /// Build final string (zero-copy if fits in original capacity)
    pub fn build(self) -> Result<Cow<'static, str>, BufferError> {
        let slice = self.buffer.as_slice();

        // Safe because buffer contains valid UTF-8 (we only add valid str slices)
        let s = unsafe { std::str::from_utf8_unchecked(slice) };

        if s.is_empty() {
            Ok(Cow::Borrowed(""))
        } else {
            // We need to allocate because the buffer is going out of scope
            Ok(Cow::Owned(s.to_string()))
        }
    }

    /// Get current string view (zero-copy)
    pub fn as_str(&self) -> &str {
        let slice = self.buffer.as_slice();
        // Safe because we only add valid UTF-8
        unsafe { std::str::from_utf8_unchecked(slice) }
    }

    /// Check if more data can be added
    pub fn can_fit(&self, additional: usize) -> bool {
        self.buffer.len() + additional <= N
    }

    /// Remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        N - self.buffer.len()
    }
}

/// **REFERENCE-COUNTED IMMUTABLE BUFFER**
///
/// Share buffer data without cloning using reference counting
use std::sync::Arc;

pub struct SharedBuffer {
    data: Arc<[u8]>,
    offset: usize,
    len: usize,
}

impl SharedBuffer {
    /// Create from owned data
    pub fn new(data: Vec<u8>) -> Self {
        let len = data.len();
        Self {
            data: Arc::from(data.into_boxed_slice()),
            offset: 0,
            len,
        }
    }

    /// Create view of subset (zero-copy)
    pub fn slice(&self, start: usize, end: usize) -> Result<Self, BufferError> {
        if start > end || end > self.len {
            return Err(BufferError::InvalidRange {
                start,
                end,
                len: self.len,
            });
        }

        Ok(Self {
            data: Arc::clone(&self.data),
            offset: self.offset + start,
            len: end - start,
        })
    }

    /// Get data as slice (zero-copy)
    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..self.offset + self.len]
    }

    /// Length of this view
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get reference count (for debugging)
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }
}

impl Clone for SharedBuffer {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            offset: self.offset,
            len: self.len,
        }
    }
}

/// **LIFETIME-AWARE CONFIGURATION**
///
/// Configuration that borrows when possible, owns when necessary
pub enum Config<'a> {
    Borrowed(&'a ConfigData),
    Owned(Box<ConfigData>),
}

pub struct ConfigData {
    pub name: String,
    pub value: String,
    pub category: String,
}

impl<'a> Config<'a> {
    /// Create from borrowed data (zero allocation)
    pub fn from_borrowed(data: &'a ConfigData) -> Self {
        Config::Borrowed(data)
    }

    /// Create from owned data (single allocation)
    pub fn from_owned(data: ConfigData) -> Self {
        Config::Owned(Box::new(data))
    }

    /// Get reference to data (zero-copy)
    pub fn data(&self) -> &ConfigData {
        match self {
            Config::Borrowed(data) => data,
            Config::Owned(data) => data,
        }
    }

    /// Convert to owned version if not already
    pub fn into_owned(self) -> Config<'static> {
        match self {
            Config::Borrowed(data) => Config::Owned(Box::new(ConfigData {
                name: data.name.clone(),
                value: data.value.clone(),
                category: data.category.clone(),
            })),
            Config::Owned(data) => Config::Owned(data),
        }
    }
}

/// **CONST GENERIC UTILITIES**
/// Compile-time optimizations using const generics
/// Optimal buffer size for different operations (compile-time known)
pub trait OptimalBufferSize {
    const SIZE: usize;

    type Buffer;

    fn create_buffer() -> Self::Buffer;
}

/// Command output buffer size
pub struct CommandOutput;
impl OptimalBufferSize for CommandOutput {
    const SIZE: usize = 16384;
    type Buffer = AlignedBuffer<{ Self::SIZE }>;

    fn create_buffer() -> Self::Buffer {
        AlignedBuffer::new()
    }
}

/// Network I/O buffer size  
pub struct NetworkIo;
impl OptimalBufferSize for NetworkIo {
    const SIZE: usize = 65536;
    type Buffer = AlignedBuffer<{ Self::SIZE }>;

    fn create_buffer() -> Self::Buffer {
        AlignedBuffer::new()
    }
}

/// File I/O buffer size
pub struct FileIo;
impl OptimalBufferSize for FileIo {
    const SIZE: usize = 1048576;
    type Buffer = AlignedBuffer<{ Self::SIZE }>;

    fn create_buffer() -> Self::Buffer {
        AlignedBuffer::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aligned_buffer() {
        let mut buffer: AlignedBuffer<64> = AlignedBuffer::new();
        assert_eq!(AlignedBuffer::<64>::capacity(), 64);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());

        let data = b"Hello, world!";
        let result = buffer.init_from_slice(data).unwrap();
        assert_eq!(result, data);
        assert_eq!(buffer.len(), data.len());
    }

    #[test]
    fn test_stack_string_builder() {
        let mut builder: StackStringBuilder<64> = StackStringBuilder::new();

        builder.push_str("Hello").unwrap();
        builder.push_str(", ").unwrap();
        builder.push_str("world!").unwrap();

        assert_eq!(builder.as_str(), "Hello, world!");

        let result = builder.build().unwrap();
        assert_eq!(result.as_ref(), "Hello, world!");
    }

    #[test]
    fn test_shared_buffer() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = SharedBuffer::new(data);

        let slice1 = buffer.slice(1, 4).unwrap();
        let slice2 = buffer.slice(2, 5).unwrap();

        assert_eq!(slice1.as_slice(), &[2, 3, 4]);
        assert_eq!(slice2.as_slice(), &[3, 4, 5]);
        assert_eq!(buffer.ref_count(), 3); // Original + 2 slices
    }
}
