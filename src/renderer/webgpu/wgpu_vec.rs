use super::WGPUContext;

pub struct WGPUVec<T: Copy> {
    cpu: Vec<T>,
    gpu: wgpu::Buffer,
    ph: std::marker::PhantomData<T>,
}

impl<T: Copy> WGPUVec<T> {
    pub fn new(ctx: WGPUContext) -> Self {
        // Self {
        //     cpu: vec![],
        //     gpu:
        // }
        todo!()
    }

    pub fn len(&self) -> usize {
        self.cpu.len()
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        self.cpu.extend_from_slice(other);
    }

    pub fn upload(&mut self) {
        self.gpu.destroy()
    }

    pub fn as_slice<'a>(&'a self) -> wgpu::BufferSlice<'a> {
        self.gpu.slice(0..0)
    }

    // pub fn as_mut_slice<'a>(&'a mut self) -> wgpu::BufferMutSlice<'a> {
    //     todo!()
    // }
}

impl<T: Copy> Drop for WGPUVec<T> {
    fn drop(&mut self) {
        let a = vec![1];

        self.gpu.destroy()
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
