
// pub(crate)
pub fn round_up(x: usize, to: usize) -> usize {
    let m = x % to;
    if m == 0 {
        x
    } else {
        x - m + to
    }
}

// pub(crate)
pub fn page_aligned(size: usize) -> usize {
    round_up(size, 4096)
}

/// `MemAlign` represents metadata for a page alligned allocation.
///
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct MemAlign<T> {
    pub byte_size: usize,
    pub capacity: usize,
    pub remainder: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<T> MemAlign<T> {
    pub fn element_size() -> usize {
        std::mem::size_of::<T>()
    }

    pub fn is_valid(&self) -> bool {
        (Self::element_size() * self.capacity) + self.remainder == self.byte_size
    }

    pub fn new(capacity: usize) -> Self {
        let element_size = Self::element_size();
        let size = element_size * capacity;

        let byte_size = page_aligned(size);
        let remainder = byte_size % element_size;
        assert!((byte_size - remainder) % element_size == 0);
        let capacity = (byte_size - remainder) / element_size;
        assert!(byte_size != 0);

        Self {
            byte_size,
            capacity,
            remainder,
            phantom: Default::default(),
        }
    }
}