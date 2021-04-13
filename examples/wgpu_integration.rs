fn main() {
    #[cfg(feature="wgpu-renderer")]
    integration::integration_main();
}

#[cfg(feature="wgpu-renderer")]
mod integration {

use std::rc::Rc;

use winit::event::{
    Event,
    WindowEvent,
};
use winit::event_loop::{
    ControlFlow,
    EventLoop,
};

use femtovg::{
    renderer::{
        WGPUContext,
        WGPUInstance,
        WGPUSwapChain,
        WGPU,
    },
    Align,
    Baseline,
    Canvas,
    Color,
    FillRule,
    FontId,
    ImageFlags,
    ImageId,
    LineCap,
    LineJoin,
    Paint,
    Path,
    Renderer,
    Size,
    Solidity,
};

pub fn integration_main() {
    let event_loop = EventLoop::new();
    let size = winit::dpi::LogicalSize::new(1024, 800);
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("wgpu integration")
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(event_loop, window));
}

async fn run(event_loop: EventLoop<()>, window: winit::window::Window) {
    let size = window.inner_size();

    // Normal wgpu structs ---------------------------

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();
    
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::PUSH_CONSTANTS,
                limits: wgpu::Limits {
                    max_push_constant_size: 4096, // This must be at-least 8
                    ..wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        )
        .await
        .unwrap();

    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);
    

    // Femtovg integration -----------------------------
    
    let instance = Rc::new(instance);
    let surface = Rc::new(surface);
    let adapter = Rc::new(adapter);
    let device = Rc::new(device);
    let queue = Rc::new(queue);
    
    let instance = WGPUInstance::from_instance(Rc::clone(&instance), Rc::clone(&adapter), Some(Rc::clone(&surface)));
    let ctx = WGPUContext::from_device(instance, Rc::clone(&device), Rc::clone(&queue));
    let size = Size::new(size.width as _, size.height as _);
    let renderer = WGPU::new(&ctx, size, wgpu::TextureFormat::Bgra8Unorm);

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                #[cfg(not(target_arch = "wasm32"))]
                WindowEvent::Resized(new_size) => {
                    let new_size = Size::new(new_size.width as _, new_size.height as _);
                    canvas.set_size(new_size.w as _, new_size.h as _, 1.0);

                    let sc_desc = wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                        format: wgpu::TextureFormat::Bgra8Unorm,
                        width: new_size.w as _,
                        height: new_size.h as _,
                        present_mode: wgpu::PresentMode::Fifo,
                    };
                    swap_chain = device.create_swap_chain(&surface, &sc_desc);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // Normal wgpu rendering  ------------------------------

                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(_) => {
                        // The swapchain is outdated. Try again next frame.
                        window.request_redraw();
                        return;
                    }
                };

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                {
                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[
                            wgpu::RenderPassColorAttachment {
                                view: &frame.output.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.1,
                                        g: 0.2,
                                        b: 0.3,
                                        a: 1.0,
                                    }),
                                    store: true,
                                }
                            }
                        ],
                        depth_stencil_attachment: None,
                    });
                }
            
                queue.submit(std::iter::once(encoder.finish()));
            

                // Femtovg rendering  -----------------------------------

                canvas.clear_rect(100, 100, 100, 100, Color::rgbf(1.0, 0.0, 0.0));

                let target = &frame.output.view;
                canvas.flush(Some(target));
            }
            Event::MainEventsCleared => {
                window.request_redraw()
            }
            _ => (),
        }
    });
}

}