use std::future::Future;
pub struct WGPUInstance {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
}

impl WGPUInstance {
    // pub fn from_window(window: &winit::window::Window) -> impl Future<Output = Result<Self, wgpu::RequestDeviceError>>  {
    pub fn from_window(window: &winit::window::Window) -> impl Future<Output = Option<Self>> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        });
        async move { adapter.await.map(|adapter| Self { instance, adapter }) }
    }
}

#[derive(Clone)]
pub struct WGPUContext {
    device: std::rc::Rc<wgpu::Device>,
    queue: std::rc::Rc<wgpu::Queue>,
}

impl WGPUContext {
    pub fn new() -> Self {
        todo!()
    }
}

impl WGPUContext {
    pub fn device(&self) -> &std::rc::Rc<wgpu::Device> {
        &self.device
    }

    pub fn queue(&self) -> &std::rc::Rc<wgpu::Queue> {
        &self.queue
    }
}
// #[derive(Clone)]
// pub struct WGPUDevice {
//     inner: std::rc::Rc<wgpu::Device>
// }
