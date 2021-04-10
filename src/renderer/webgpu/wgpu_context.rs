use std::rc::Rc;

use raw_window_handle::HasRawWindowHandle;

use std::future::Future;
#[derive(Clone)]
pub struct WGPUInstance {
    pub instance: Rc<wgpu::Instance>,
    pub adapter: Rc<wgpu::Adapter>,
    pub surface: Option<Rc<wgpu::Surface>>,
}

impl WGPUInstance {
    pub fn from_window(window: &impl HasRawWindowHandle) -> impl Future<Output = Option<Self>> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        });
        async move {
            adapter.await.map(|adapter| Self {
                instance: Rc::new(instance),
                adapter: Rc::new(adapter),
                surface: Some(Rc::new(surface)),
            })
        }
    }

    pub fn from_instance(
        instance: Rc<wgpu::Instance>,
        adapter: Rc<wgpu::Adapter>,
        surface: Option<Rc<wgpu::Surface>>,
    ) -> Self {
        Self {
            instance,
            adapter,
            surface,
        }
    }

    pub fn new() -> impl Future<Output = Option<Self>> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default());

        async move {
            adapter.await.map(|adapter| Self {
                instance: Rc::new(instance),
                adapter: Rc::new(adapter),
                surface: None,
            })
        }
    }
}

pub trait WGPUQueueExt {
    fn sync_buffer<T: Copy>(&self, buffer: &wgpu::Buffer, data: &[T]);
}

impl WGPUQueueExt for wgpu::Queue {
    fn sync_buffer<T: Copy>(&self, buffer: &wgpu::Buffer, data: &[T]) {
        // println!("before");
        self.write_buffer(buffer, 0, super::as_u8_slice(data));
        // println!("after");
    }
}

#[derive(Clone)]
pub struct WGPUContext {
    pub instance: WGPUInstance,
    pub device: Rc<wgpu::Device>,
    pub queue: Rc<wgpu::Queue>,
}

impl WGPUContext {
    pub fn new(instance: WGPUInstance) -> impl Future<Output = Result<Self, wgpu::RequestDeviceError>> {
        // instance.adapter.request_device(desc, trace_path)
        // let path = std::path::Path::new("/Users/adamnemecek/Code/femtovg3/tracing");
        let f = instance.adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::PUSH_CONSTANTS,
                limits: wgpu::Limits {
                    max_push_constant_size: 4096,
                    ..Default::default()
                },
            },
            // Some(path),
            None,
        );
        async move {
            f.await.map(|(device, queue)| Self {
                instance: instance.clone(),
                device: Rc::new(device),
                queue: Rc::new(queue),
            })
        }
    }

    pub fn from_device(instance: WGPUInstance, device: Rc<wgpu::Device>, queue: Rc<wgpu::Queue>) -> Self {
        Self {
            instance,
            device,
            queue
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

    #[inline]
    pub fn create_command_encoder(&self, label: Option<&str>) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label })
    }

    pub fn get_swap_chain_preferred_format(&self) -> wgpu::TextureFormat {
        let format = self.adapter().get_swap_chain_preferred_format(self.surface()).unwrap();
        format
    }
}
// #[derive(Clone)]
// pub struct WGPUDevice {
//     inner: Rc<wgpu::Device>
// }
