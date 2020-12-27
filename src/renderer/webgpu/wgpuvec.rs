pub struct WGPUVec<T: Copy> {
    inner: wgpu::Buffer,
    marker: std::marker::PhantomData<T>,
}

impl<T: Copy> WGPUVec<T> {
    pub fn new(device: &wgpu::Device, capacity: usize) -> Self {
        // let inner = device.create_buffer(wgpu::BufferDescriptor {
        //     label: Some("fds"),
        //     size:
        // });
        // Self {
        //    inner,
        //    marker: Default::default(),
        // }
        todo!()
    }

    pub fn inner(&self) -> &wgpu::Buffer {
        &self.inner
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn resize(&mut self, size: usize) {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn extend_from_slice(&mut self, slice: &[T]) {
        todo!()
    }


}

impl<T: Copy> AsRef<T> for WGPUVec<T> {
    fn as_ref(&self) -> &T {
        todo!()
    }
}

impl<T: Copy> std::ops::Index<usize> for WGPUVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T: Copy> std::ops::IndexMut<usize> for WGPUVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // pub fn set(&mut self, index: usize, v: T) {
        //     let mut z = self.inner.slice(..).get_mapped_range_mut();

        //     unsafe {
        //         let ptr = z.as_mut_ptr();
        //         ptr.offset((index * std::mem::size_of::<T>()) as isize);
        //     }

        // }
        todo!()
    }
}
