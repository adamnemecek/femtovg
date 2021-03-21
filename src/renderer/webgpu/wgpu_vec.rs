use super::{
    MemAlign,
    WGPUContext,
};

pub struct WGPUVec<T: Copy> {
    // cpu: Vec<T>,
    inner: wgpu::Buffer,
    len: usize,
    mem_align: MemAlign<T>,
}

impl<T: Copy> WGPUVec<T> {
    pub fn new(ctx: &WGPUContext) -> Self {
        // Self {
        //     cpu: vec![],
        //     gpu:
        // }
        todo!()
    }

    pub fn len(&self) -> usize {
        // self.cpu.len()
        todo!()
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        // self.cpu.extend_from_slice(other);
        todo!()
    }

    pub fn resize(&mut self, capacity: usize) {
        if capacity <= self.capacity() {
            return;
        }
        let mem_align = MemAlign::<T>::new(capacity);
        // let inner = self.device.new_mem(
        //     mem_align,
        //     metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        // );
        // unsafe {
        //     std::ptr::copy(
        //         self.as_ptr(),
        //         // inner.contents() as *mut T,
        //         inner.as_mut_ptr(),
        //         self.len(),
        //     );
        // }
        self.mem_align = mem_align;
        // self.inner = inner;
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        // self.inner.slice(bounds)
        todo!()
    }

    pub fn capacity(&self) -> usize {
        self.mem_align.capacity
    }

    pub fn upload(&mut self) {
        // self.gpu.destroy()
        todo!()
    }

    pub fn as_raw(&self) -> &wgpu::Buffer {
        &self.inner
    }

    pub fn as_slice<S: std::ops::RangeBounds<wgpu::BufferAddress>>(&self, bounds: S) -> wgpu::BufferSlice {
        self.inner.slice(bounds)
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    // pub fn as_slice<'a>(&'a self) -> wgpu::BufferSlice<'a> {
    //     // self.gpu.slice(0..0)
    //     todo!()
    // }

    // pub fn as_mut_slice<'a>(&'a mut self) -> wgpu::BufferMutSlice<'a> {
    //     todo!()
    // }
}

impl<T: Copy> Drop for WGPUVec<T> {
    fn drop(&mut self) {
        self.inner.destroy()
    }
}

impl WGPUVec<u32> {
    pub fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize {
        let mut added = 0;
        for index in 1..(count - 1) {
            self.extend_from_slice(&[start, start + index, start + index + 1]);
            added += 3;
        }

        added
    }
}
