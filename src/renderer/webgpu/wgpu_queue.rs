use std::future::Future;
#[derive(Clone)]
pub struct WGPUInstance {
    instance: std::rc::Rc<wgpu::Instance>,
    adapter: std::rc::Rc<wgpu::Adapter>,
    surface: Option<std::rc::Rc<wgpu::Surface>>,
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
        async move {
            adapter.await.map(|adapter| Self {
                instance: std::rc::Rc::new(instance),
                adapter: std::rc::Rc::new(adapter),
                surface: Some(std::rc::Rc::new(surface)),
            })
        }
    }

    pub fn new() -> impl Future<Output = Option<Self>> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default());

        async move {
            adapter.await.map(|adapter| Self {
                instance: std::rc::Rc::new(instance),
                adapter: std::rc::Rc::new(adapter),
                surface: None,
            })
        }
    }
}
#[derive(Clone)]
pub struct WGPUContext {
    instance: WGPUInstance,
    device: std::rc::Rc<wgpu::Device>,
    queue: std::rc::Rc<wgpu::Queue>,
}

impl WGPUContext {
    pub fn new(instance: WGPUInstance) -> impl Future<Output = Result<Self, wgpu::RequestDeviceError>> {
        // instance.adapter.request_device(desc, trace_path)
        let f = instance.adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::PUSH_CONSTANTS,
                limits: wgpu::Limits {
                    max_push_constant_size: 4096,
                    ..Default::default()
                },
            },
            None,
        );
        async move {
            f.await.map(|(device, queue)| Self {
                instance: instance.clone(),
                device: std::rc::Rc::new(device),
                queue: std::rc::Rc::new(queue),
            })
        }
    }
}

impl WGPUContext {
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.instance.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&self) -> &wgpu::Surface {
        self.instance.surface.as_ref().unwrap()
    }

    pub fn get_swap_chain_preferred_format(&self) -> wgpu::TextureFormat {
        self.adapter()
                .get_swap_chain_preferred_format(self.surface())
                .unwrap()
    }
}
// #[derive(Clone)]
// pub struct WGPUDevice {
//     inner: std::rc::Rc<wgpu::Device>
// }
