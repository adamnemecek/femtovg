
pub struct WGPUVar<T> {
    inner: wgpu::Buffer,
    ph: std::marker::PhantomData<T>,
}

impl<T> WGPUVar<T> {

}

impl<T> std::ops::Deref for WGPUVar<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let slice = self.inner.slice(..);
        let z = slice.get_mapped_range();
        let z = z.as_ref();
        unsafe {
            (z.as_ptr() as *const T).as_ref().unwrap()
        }
    }
}