use wgpu::util::{
    BufferInitDescriptor,
    DeviceExt,
};

pub struct WGPUVar<T> {
    inner: wgpu::Buffer,
    ph: std::marker::PhantomData<T>,
}

fn as_u8_slice<T>(v: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}

fn pipeline(pipeline: &wgpu::RenderPipeline) {
    let layout = pipeline.get_bind_group_layout(0);
    // layout.
    todo!()
}

impl<T> WGPUVar<T> {
    pub fn new(device: &wgpu::Device, label: Option<&str>, value: &T, usage: wgpu::BufferUsage) -> Self {
        // let c =

        let inner = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents: as_u8_slice(value),
            usage,
        });
        Self {
            inner,
            ph: Default::default(),
        }
    }
}

impl<T> std::ops::Deref for WGPUVar<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let slice = self.inner.slice(..);
        let z = slice.get_mapped_range();
        let z = z.as_ref();
        unsafe { (z.as_ptr() as *const T).as_ref().unwrap() }
    }
}

impl<T> std::ops::DerefMut for WGPUVar<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let slice = self.inner.slice(..);
        let z = slice.get_mapped_range_mut();
        let z = z.as_ref();
        unsafe { (z.as_ptr() as *mut T).as_mut().unwrap() }
    }
}

mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct Test {
        a: u32,
    }

    fn test() {

        // let mut var = WGPUVar::new();
    }
}
