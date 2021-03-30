mod wgpu_vec;
use wgpu::util::RenderEncoder;
pub use wgpu_vec::*;

mod wgpu_context;
pub use wgpu_context::*;

mod wgpu_texture;
pub use wgpu_texture::*;

mod wgpu_stencil_texture;
pub use wgpu_stencil_texture::*;

mod wgpu_ext;
pub use wgpu_ext::*;

mod wgpu_pipeline_cache;
pub use wgpu_pipeline_cache::*;

mod mem_align;
pub use mem_align::*;

mod wgpu_swap_chain;
pub use wgpu_swap_chain::*;

mod wgpu_bind_group_cache;
pub use wgpu_bind_group_cache::*;

mod wgpu_var;
pub use wgpu_var::*;

use crate::{
    renderer::{
        ImageId,
        Vertex,
    },
    BlendFactor,
    Color,
    CompositeOperationState,
    ErrorKind,
    FillRule,
    ImageInfo,
    ImageSource,
    ImageStore,
    Rect,
    Size,
};

use super::{
    Command,
    CommandType,
    Params,
    RenderTarget,
    Renderer,
};

use self::VecExt;

// use fnv::FnvHashMap;
use imgref::ImgVec;
use rgb::RGBA8;
use std::borrow::Cow;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct WGPUBlend {
    pub src_rgb: wgpu::BlendFactor,
    pub dst_rgb: wgpu::BlendFactor,
    pub src_alpha: wgpu::BlendFactor,
    pub dst_alpha: wgpu::BlendFactor,
}

impl From<BlendFactor> for wgpu::BlendFactor {
    fn from(a: BlendFactor) -> Self {
        match a {
            BlendFactor::Zero => Self::Zero,
            BlendFactor::One => Self::One,
            BlendFactor::SrcColor => Self::SrcColor,
            BlendFactor::OneMinusSrcColor => Self::OneMinusSrcColor,
            BlendFactor::DstColor => Self::DstColor,
            BlendFactor::OneMinusDstColor => Self::OneMinusDstColor,
            BlendFactor::SrcAlpha => Self::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => Self::OneMinusSrcAlpha,
            BlendFactor::DstAlpha => Self::DstAlpha,
            BlendFactor::OneMinusDstAlpha => Self::OneMinusDstAlpha,
            BlendFactor::SrcAlphaSaturate => Self::SrcAlphaSaturated,
        }
    }
}

impl From<CompositeOperationState> for WGPUBlend {
    fn from(v: CompositeOperationState) -> Self {
        Self {
            src_rgb: v.src_rgb.into(),
            dst_rgb: v.dst_rgb.into(),
            src_alpha: v.src_alpha.into(),
            dst_alpha: v.dst_alpha.into(),
        }
    }
}

fn begin_render_pass<'a>(
    // ctx: WGPUContext,
    encoder: &'a mut wgpu::CommandEncoder,
    target: &'a wgpu::TextureView,
    view_size: Size,
    // images: &'a ImageStore<WGPUTexture>,
    // command_buffer: &'a wgpu::CommandBuffer,
    clear_color: wgpu::Color,
    // stencil_texture: &'a mut WGPUStencilTexture,
    stencil_view: &'a wgpu::TextureView,
    vertex_buffer: &'a WGPUVec<Vertex>,
    index_buffer: &'a WGPUVec<u32>,
    // uniform_buffer: &'a WGPUVec<Params>,

    // ) -> wgpu::CommandEncoder {
) -> wgpu::RenderPass<'a> {
    // stencil_texture.resize(view_size);

    let pass_desc = wgpu::RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: target,
            resolve_target: None, // todo! what's this?
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: true,
            },
        }],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
            // attachment: stencil_texture.view(),
            attachment: stencil_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(0.0),
                store: true,
            }),
            // todo: what is this?
            stencil_ops: None,
            // stencil_ops: Some(wgpu::Operations {
            //     load: wgpu::LoadOp::Clear(0),
            //     store: true,
            // }), //Option<Operations<u32>>,
        }),
    };

    // todo set cull mode on the state

    // let mut encoder = ctx
    //     .device()
    //     .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let mut pass = encoder.begin_render_pass(&pass_desc);
    pass.set_viewport(0.0, 0.0, view_size.w as _, view_size.h as _, 0.0, 1.0);

    pass.set_vertex_buffer(0, vertex_buffer.as_ref().slice(..));
    pass.set_stencil_reference(0);

    pass.set_index_buffer(index_buffer.as_ref().slice(..), wgpu::IndexFormat::Uint32);

    // pass.set_vertex_buffer(1, buffer_slice);
    pass

    // encoder.set_vertex_buffer(0, vertex_buffer.as_slice());
    // encoder

    // encoder
}

/// the things that
pub struct WGPU {
    ctx: WGPUContext,
    antialias: bool,
    stencil_texture: WGPUStencilTexture,

    vertex_buffer: WGPUVec<Vertex>,

    index_buffer: WGPUVec<u32>,
    temp_index_buffer: Vec<u32>,
    index_ranges: Vec<IndexRange>,

    uniform_buffer: WGPUVec<Params>,
    temp_uniform_buffer: Vec<Params>,

    render_target: RenderTarget,
    pseudo_texture: WGPUTexture,

    pipeline_cache: WGPUPipelineCache,
    bind_group_cache: WGPUBindGroupCache,
    clear_color: Color,
    view_size: Size,
    swap_chain: WGPUSwapChain,
    bind_group_layout: wgpu::BindGroupLayout,
    dpi: f32,

    temp_clear_rect_buffer: Vec<ClearRect>,
    clear_rect_buffer: WGPUVec<ClearRect>,
    clear_rect_bind_group: wgpu::BindGroup,
    clear_rect_bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Clone, Copy)]
struct IndexRange {
    start: u32,
    end: u32,
    e_start: usize,
    e_count: usize,
}

impl From<IndexRange> for std::ops::Range<u32> {
    fn from(a: IndexRange) -> Self {
        a.start..a.end
    }
}

impl WGPU {
    pub fn new(ctx: &WGPUContext, view_size: Size) -> Self {
        let default_stencil_state = 0;

        // let clear_stencil_state = {
        //     let front = wgpu::StencilFaceState {
        //         compare: wgpu::CompareFunction::Always,
        //         fail_op: wgpu::StencilOperation::Keep,
        //         depth_fail_op: wgpu::StencilOperation::Keep,
        //         pass_op: wgpu::StencilOperation::Keep,
        //     };

        //     let state = wgpu::DepthStencilState {
        //         format: wgpu::TextureFormat::Depth32Float,
        //         depth_write_enabled: false,
        //         depth_compare: wgpu::CompareFunction::LessEqual,
        //         stencil: wgpu::StencilState {
        //             front,
        //             //todo: is default the as None?
        //             back: Default::default(),
        //             read_mask: 0,
        //             write_mask: 0,
        //         },
        //         bias: Default::default(),
        //         clamp_depth: false,
        //     };
        // };

        let bind_group_layout = ctx.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[
                //viewsize
                // wgpu::BindGroupLayoutEntry {
                //     binding: 0,
                //     visibility: wgpu::ShaderStage::VERTEX,
                //     ty: wgpu::BindingType::Buffer {
                //         ty: wgpu::BufferBindingType::Uniform,
                //         has_dynamic_offset: false,
                //         min_binding_size: None,
                //     },
                //     count: None,
                // },
                // //uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Params>() as _),
                    },
                    count: None,
                },
                // texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
                // alpha texture
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                //alpha sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        });

        // let view_size_size: u32 = std::mem::size_of::<Size>() as _;
        // let param_size: u32 = std::mem::size_of::<Params>() as _;
        // println!("param_size {:?}", param_size);

        let pipeline_layout = ctx.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[
                wgpu::PushConstantRange {
                    stages: wgpu::ShaderStage::VERTEX,
                    range: 0..(std::mem::size_of::<Size>() as _),
                },
                // wgpu::PushConstantRange {
                //     stages: wgpu::ShaderStage::FRAGMENT,
                //     range: view_size_size..(view_size_size + param_size),
                // },
                // wgpu::PushConstantRange {
                //     stages: wgpu::ShaderStage::FRAGMENT,
                //     range: (view_size_size + param_size)..(view_size_size + param_size),
                // },
            ],
        });

        let clear_rect_bind_group_layout = ctx.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("clear rect bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<ClearRect>() as _),
                },
                count: None,
            }],
        });

        let clear_rect_pipeline_layout = ctx.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("clear rect pipeline layout"),
            bind_group_layouts: &[&clear_rect_bind_group_layout],
            push_constant_ranges: &[
                // wgpu::PushConstantRange {
                //     stages: wgpu::ShaderStage::VERTEX,
                //     range: 0..view_size_size,
                // },
            ],
        });

        let clear_rect_buffer = WGPUVec::new_uniform(ctx, 16);

        let clear_rect_bind_group =
            self::create_clear_rect_bind_group(ctx, &clear_rect_bind_group_layout, &clear_rect_buffer);

        // let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[wgpu::BindGroupLayoutEntry {
        //         binding: 0,
        //         visibility: wgpu::ShaderStage::FRAGMENT,
        //         ty: wgpu::BindingType::Texture {
        //             sample_type: wgpu::TextureSampleType::Float { filterable: true },
        //             view_dimension: wgpu::TextureViewDimension::D2,
        //             multisampled: false,
        //         },
        //         count: std::num::NonZeroU32::new(2),
        //     }],
        // });

        // vertex shader
        //  * vertex
        //  * viewsize
        // fragment shader

        // let encoder = ctx
        //     .device()
        //     .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let stencil_texture = WGPUStencilTexture::new(ctx, view_size);
        let vertex_buffer = WGPUVec::new_vertex(ctx, 1024);
        let index_buffer = WGPUVec::new_index(ctx, 1024);
        let uniform_buffer = WGPUVec::new_uniform(ctx, 32);

        let mut flags = wgpu::ShaderFlags::VALIDATION;
        match ctx.adapter().get_info().backend {
            wgpu::Backend::Metal | wgpu::Backend::Vulkan => flags |= wgpu::ShaderFlags::EXPERIMENTAL_TRANSLATION,
            _ => (), //TODO
        }

        let shader = ctx.device().create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("webgpu/shader.wgsl"))),
            flags,
        });

        let clear_color = Color::white();
        let pipeline_cache = WGPUPipelineCache::new(ctx, pipeline_layout, clear_rect_pipeline_layout, shader);
        let bind_group_cache = WGPUBindGroupCache::new();
        let swap_chain = WGPUSwapChain::new(ctx, view_size);
        let pseudo_texture = WGPUTexture::new_pseudo_texture(ctx).unwrap();

        Self {
            dpi: 1.0,
            clear_color,
            antialias: true,
            stencil_texture,
            ctx: ctx.clone(),

            vertex_buffer,

            index_buffer,
            temp_index_buffer: vec![],
            index_ranges: vec![],

            uniform_buffer,
            temp_uniform_buffer: vec![],

            render_target: RenderTarget::Screen,
            pseudo_texture,
            pipeline_cache,
            bind_group_cache,
            view_size,
            bind_group_layout,
            swap_chain,

            clear_rect_buffer,
            temp_clear_rect_buffer: vec![],
            clear_rect_bind_group_layout,
            clear_rect_bind_group,
        }
    }
}

// fn start_capture() {
//     let device = metal::Device::system_default().unwrap();
//     let z = metal::CaptureManager::shared();
//     z.start_capture_with_device(&device);
// }

// fn stop_capture() {
//     let device = metal::Device::system_default().unwrap();
//     let z = metal::CaptureManager::shared();
//     z.stop_capture();
// }

impl WGPU {
    // pub fn bind_group_for(
    //     &self,
    //     images: &ImageStore<WGPUTexture>,
    //     image_tex: Option<ImageId>,
    //     alpha_tex: Option<ImageId>,
    // ) -> &WGPUBindGroup {
    //     self.bind_group_cache.get(
    //         &self.ctx,
    //         images,
    //         &self.bind_group_layout,
    //         image_tex,
    //         alpha_tex,
    //         &self.pseudo_texture,
    //     )
    // }
}

#[inline]
fn vert_range(start: usize, count: usize) -> std::ops::Range<u32> {
    (start as _)..(start + count) as _
}

// enum TargetTexture<'a> {
//     Frame(wgpu::SwapChainFrame),
//     View(&'a wgpu::TextureView),
// }
// impl<'a> TargetTexture<'a> {
//     pub fn view(&'a self) -> &'a wgpu::TextureView {
//         match self {
//             Self::Frame(f) => &f.output.view,
//             Self::View(r) => r,
//         }
//     }
// }

// impl<'a> Drop for TargetTexture<'a> {
//     fn drop(&mut self) {
//         match self {
//             Self::Frame(_) => println!("dropping frame"),
//             Self::View(_) => println!("dropping view"),
//         }
//     }
// }

impl Renderer for WGPU {
    type Image = WGPUTexture;
    fn set_size(&mut self, width: u32, height: u32, dpi: f32) {
        let size = Size::new(width, height);
        println!("set size {:?}", size);
        self.view_size = size;
        self.dpi = dpi;

        // we need to flush all the bind groups since they are bound to particular
        self.bind_group_cache.clear();
        self.swap_chain.resize(size);
        self.stencil_texture.resize(size);
        // self.pipeline_cache.clear();
    }

    fn render(&mut self, images: &ImageStore<Self::Image>, verts: &[Vertex], commands: &[Command]) {
        // todo!("clear rect {:?}", std::mem::size_of::<ClearRect>());

        // println!("render start");
        // self.vertex_buffer.clear();
        // self.vertex_buffer.extend_from_slice(verts);

        // let texture_format = &self.swap_chain.format();
        // let format = texture_format.clone();
        let swap_chain_format = self.swap_chain.format();
        // println!("texture format {:?}", texture_format);

        let mut render_target = self.render_target;

        // self.ctx.device().create_bind_group()
        // let mut texture_format = target_texture.format();

        // let pass = new_render_pass(
        //     &mut encoder,
        //     target,
        //     command_buffer,
        //     clear_color,
        //     stencil_texture,
        //     vertex_buffer,
        //     view_size,
        // );
        // let mut pass = new_pass();
        // let mut state: Option<WGPUPipelineState> = None;
        let mut prev_states: Option<&WGPUPipelineStates> = None;
        // let mut prev_bind_group: Option<&WGPUBindGroup> = None;

        // let bind_groups = vec![];
        let mut uniforms_offset: u32 = 0;
        let mut clear_rect_uniform_offset: u32 = 0;

        // let mut current_frame = None;

        self.temp_index_buffer.clear();
        self.temp_uniform_buffer.clear();
        self.temp_clear_rect_buffer.clear();

        println!("len {:?}", self.temp_uniform_buffer.len());

        self.index_ranges.clear();

        let start = std::time::Instant::now();

        // process indices
        for cmd in commands.iter() {
            match cmd.cmd_type {
                CommandType::ConvexFill { params } => {
                    // println!("set uniforms convex fill buffer");

                    self.temp_uniform_buffer.push(params);
                    // println!("len {:?}", self.temp_uniform_buffer.len());

                    for drawable in &cmd.drawables {
                        if let Some((start, count)) = drawable.fill_verts {
                            let index_range_start = self.temp_index_buffer.len();

                            self.temp_index_buffer
                                .extend_with_triange_fan_indices_cw(start as _, count as _);

                            let index_range_end = self.temp_index_buffer.len();

                            self.index_ranges.push(IndexRange {
                                start: index_range_start as _,
                                end: index_range_end as _,
                                e_start: start,
                                e_count: count,
                            });
                        }
                    }
                }
                CommandType::ConcaveFill {
                    fill_params,
                    stencil_params,
                } => {
                    // println!("set uniforms concave fill buffer");
                    self.temp_uniform_buffer.push(stencil_params);
                    self.temp_uniform_buffer.push(fill_params);
                    // println!("len {:?}", self.temp_uniform_buffer.len());

                    for drawable in &cmd.drawables {
                        if let Some((start, count)) = drawable.fill_verts {
                            let index_range_start = self.temp_index_buffer.len();

                            self.temp_index_buffer
                                .extend_with_triange_fan_indices_cw(start as _, count as _);

                            let index_range_end = self.temp_index_buffer.len();

                            self.index_ranges.push(IndexRange {
                                start: index_range_start as _,
                                end: index_range_end as _,
                                e_start: start,
                                e_count: count,
                            });
                        }
                    }
                }
                CommandType::Stroke { params } => {
                    // println!("set uniforms stroke");
                    self.temp_uniform_buffer.push(params);
                    println!("len {:?}", self.temp_uniform_buffer.len());
                }
                CommandType::StencilStroke { params1, params2 } => {
                    // println!("set uniforms stencil stroke");
                    self.temp_uniform_buffer.push(params2);
                    self.temp_uniform_buffer.push(params1);
                    // println!("len {:?}", self.temp_uniform_buffer.len());
                }
                CommandType::Triangles { params } => {
                    // println!("set uniforms triangles");
                    self.temp_uniform_buffer.push(params);
                    // println!("len {:?}", self.temp_uniform_buffer.len());
                }
                CommandType::ClearRect {
                    // x,
                    // y,
                    // width,
                    // height,
                    color,
                    ..
                } => {
                    // println!("set uniforms clear rect");
                    self.temp_clear_rect_buffer.push({
                        let rect = Rect {
                            x: -1.0,
                            y: -1.0,
                            w: 2.0,
                            h: 2.0,
                        };
                        ClearRect::new(rect, color)
                    })
                }
                CommandType::SetRenderTarget(_) => {}
            }
        }

        for (i, e) in self.temp_uniform_buffer.iter().enumerate() {
            println!("{:?} inner {:?}", i, e.inner_col);
        }

        // for cmd in commands.iter() {
        //     println!("cmd {:?}", cmd.cmd_type);
        // }
        let end = std::time::Instant::now();
        // println!("uniforms vec {:?}", end - start);

        // println!("command count {:?}", commands.len());
        // todo!(
        //     "temp_uniforms len {:?} bytes {:?}",
        //     self.temp_uniform_buffer.len(),
        //     self.temp_uniform_buffer.len() * 256
        // );
        // println!("index len {:?}", self.temp_index_buffer.len());

        // println!("verts len {:?}", verts.len());
        {
            self.vertex_buffer.resize(verts.len());
            println!("resized to {:?}", self.vertex_buffer.capacity());
            self.ctx.queue().sync_buffer(self.vertex_buffer.as_ref(), verts);
        }

        // self.index_buffer.clear();
        // assert!()
        // self.temp_index_buffer.resize(verts.len() * 3, 0);
        {
            self.index_buffer.resize(self.temp_index_buffer.len());
            self.ctx
                .queue()
                .sync_buffer(self.index_buffer.as_ref(), &self.temp_index_buffer);
        }

        // sync uniforms
        {
            self.uniform_buffer.resize(self.temp_uniform_buffer.len());
            self.ctx
                .queue()
                .sync_buffer(self.uniform_buffer.as_ref(), &self.temp_uniform_buffer);
            // println!("index before");
        }

        // sync clear rect uniforms
        {
            if self
                .clear_rect_buffer
                .resize(self.temp_clear_rect_buffer.len())
                .resized()
            {
                self.clear_rect_bind_group = self::create_clear_rect_bind_group(
                    &self.ctx,
                    &self.clear_rect_bind_group_layout,
                    &self.clear_rect_buffer,
                );
            }

            self.ctx
                .queue()
                .sync_buffer(self.clear_rect_buffer.as_ref(), &self.temp_clear_rect_buffer);
        }

        let mut i = 0;

        let mut index_range_offset = 0;

        #[allow(unused_assignments)]
        let mut should_submit = true;

        let frame = self.swap_chain.get_current_frame().unwrap();
        let view = &frame.output.view;

        #[derive(Default, Debug)]
        struct Counter {
            convex_fill: usize,
            concave_fill: usize,
            stroke: usize,
            stencil_stroke: usize,
            triangles: usize,
            clear_rect: usize,
            set_render_target: usize,
        }

        let mut counter = Counter::default();

        // ///
        // let z = commands.first().unwrap();
        // let z = match z.cmd_type {
        //     CommandType::SetRenderTarget { .. } => true,
        //     _ => false,
        // };
        // println!("first command is set render target {:?}", z);

        'new_pass: while i < commands.len() {
            should_submit = true;

            let mut encoder = self.ctx.create_command_encoder(None);
            {
                let (target_view, stencil_view, view_size, texture_format) = match render_target {
                    RenderTarget::Screen => (view, self.stencil_texture.view(), self.view_size, swap_chain_format),
                    RenderTarget::Image(id) => {
                        let tex = images.get(id).unwrap();
                        (tex.view(), tex.stencil_view(), tex.size(), tex.format())
                    }
                };

                println!("view_size {:?}", view_size);
                println!("render target {:?}", render_target);

                let mut pass = begin_render_pass(
                    &mut encoder,
                    // target_texture_view.view(),
                    // &frame.output.view,
                    target_view,
                    view_size,
                    self.clear_color.into(),
                    // &mut self.stencil_texture,
                    stencil_view,
                    &self.vertex_buffer,
                    &self.index_buffer,
                    // &self.uniform_buffer,
                );
                // uniforms_offset += offset;
                // let mut index_buffer_view = self.index_buffer.view_mut();

                // pass.set_vertex_buffer(0, self.vertex_buffer.slice());
                // pass.set_index_buffer(self.index_buffer.slice(), wgpu::IndexFormat::Uint32);
                // };

                // let pass_desc = new_pass_descriptor();
                // let mut pass = encoder.begin_render_pass(&pass_desc);

                // pass.set_bind_group(index, bind_group, offsets)

                // encoder.begin_render_pass(des c)

                // pass.set_viewport(x, y, w, h, min_depth, max_depth)

                // let mut state = None;

                let mut offset = 0;

                macro_rules! bind_group {
                    ($self_: expr, $images: expr, $img: expr, $alpha: expr) => {
                        $self_.bind_group_cache.get(
                            &$self_.ctx,
                            $images,
                            &$self_.bind_group_layout,
                            &$self_.uniform_buffer,
                            $img,
                            $alpha,
                            &$self_.pseudo_texture,
                        );
                    };
                }

                'continued: while i < commands.len() {
                    let cmd = &commands[i];
                    i += 1;

                    // cache the pipeline states
                    let states = {
                        let blend: WGPUBlend = cmd.composite_operation.into();
                        let states = if let Some(prev_states) = prev_states {
                            if prev_states.matches(blend, texture_format) {
                                prev_states
                            } else {
                                self.pipeline_cache.get(blend, texture_format)
                            }
                        } else {
                            self.pipeline_cache.get(blend, texture_format)
                        };
                        states
                    };
                    prev_states = Some(states);

                    // pass.set_push_constants(wgpu::ShaderStage::FRAGMENT, 0, &[]);

                    // uniforms_offset += std::mem::size_of::<Params>();

                    match &cmd.cmd_type {
                        CommandType::ConvexFill { params } => {
                            counter.convex_fill += 1;

                            pass.cfg_push_debug_group("convex fill");
                            // set_uniforms
                            let s = states.convex_fill();
                            pass.set_pipeline(s.fill_buffer());

                            // let bg = self.bind_group_for(images, cmd.image, cmd.alpha_mask);

                            // set uniforms
                            let bg = bind_group!(self, images, cmd.image, cmd.alpha_mask);
                            pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                            uniforms_offset += std::mem::size_of::<Params>() as u32;

                            for drawable in &cmd.drawables {
                                pass.set_pipeline(s.fill_buffer());

                                let _ = pass.set_vertex_value(0, &view_size);

                                if let Some((start, count)) = drawable.fill_verts {
                                    // let offset = self.index_buffer.len();

                                    // let byte_index_buffer_offset = offset * std::mem::size_of::<u32>();

                                    // let triangle_fan_index_count = self
                                    //     .index_buffer
                                    //     .extend_with_triange_fan_indices_cw(start as u32, count as u32);
                                    // pass.set_index_buffer(self.index_buffer.as_ref().slice(), wgpu::IndexFormat::Uint32);
                                    // let fmt = wgpu::IndexFormat::Uint32;
                                    // pass.set_index_buffer(self.index_buffer.slice(), wgpu::IndexFormat::Uint32);

                                    // pass.draw_indexed((offset as _)..(offset + triangle_fan_index_count) as _, 0, 0..1);
                                    let index_range = self.index_ranges[index_range_offset];
                                    assert!(index_range.e_start == start);
                                    assert!(index_range.e_count == count);
                                    // let start = (start - 2) * 3;
                                    // let count = (count - 2) * 3;
                                    pass.draw_indexed(index_range.into(), 0, 0..1);
                                    index_range_offset += 1;
                                    // offset += (count as u32 - 2) * 3;
                                }
                                // draw fringes

                                if let Some((start, count)) = drawable.stroke_verts {
                                    pass.set_pipeline(s.stroke_buffer());
                                    // pass.draw(vert_range(start, count), 0..1);
                                    pass.draw(vert_range(start, count), 0..1);
                                }
                            }
                            pass.cfg_pop_debug_group();
                        }
                        CommandType::ConcaveFill {
                            stencil_params,
                            fill_params,
                        } => {
                            counter.concave_fill += 1;

                            pass.cfg_push_debug_group("concave fill");
                            let s = states.concave_fill();
                            pass.set_pipeline(s.fill_verts());

                            let _ = pass.set_vertex_value(0, &view_size);

                            // let bg = self.bind_group_for(images, cmd.image, cmd.alpha_mask);
                            // need for none, none
                            let bg = bind_group!(self, images, None, None);
                            // pass.set_bind_group(0, bg.as_ref(), &[]);
                            // uniforms_offset += pass.set_fragment_value(uniforms_offset, stencil_params);
                            pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                            uniforms_offset += std::mem::size_of::<Params>() as u32;

                            // fill verts
                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.fill_verts {
                                    // let offset = self.index_buffer.len();
                                    // self.index_buffer
                                    let index_range = self.index_ranges[index_range_offset];
                                    // .extend_with_triange_fan_indices_cw(start as _, count as _);
                                    assert!(index_range.e_start == start);
                                    assert!(index_range.e_count == count);
                                    pass.draw_indexed(index_range.into(), 0, 0..1);
                                    index_range_offset += 1;
                                    // pass.set_push_constants(stages, offset, data)
                                }
                            }
                            // pass.set_pipeline(states.concave_fill1());
                            // set_uniforms

                            // anti-aliased fragments
                            let bg = bind_group!(self, images, cmd.image, cmd.alpha_mask);
                            pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                            uniforms_offset += std::mem::size_of::<Params>() as u32;
                            // pass.set_bind_group(0, bg.as_ref(), &[]);
                            // uniforms_offset += pass.set_fragment_value(uniforms_offset, fill_params);

                            // fringes
                            if self.antialias {
                                match cmd.fill_rule {
                                    FillRule::NonZero => {
                                        pass.set_pipeline(s.fringes_nonzero());
                                    }
                                    FillRule::EvenOdd => {
                                        pass.set_pipeline(s.fringes_evenodd());
                                    }
                                }

                                for drawable in &cmd.drawables {
                                    if let Some((start, count)) = drawable.stroke_verts {
                                        pass.draw(vert_range(start, count), 0..1);
                                    }
                                }
                            }

                            // draw fills
                            // todo: can be moved into the if statement below?
                            match cmd.fill_rule {
                                FillRule::NonZero => {
                                    pass.set_pipeline(s.fills_nonzero());
                                }
                                FillRule::EvenOdd => {
                                    pass.set_pipeline(s.fills_evenodd());
                                }
                            }

                            if let Some((start, count)) = cmd.triangles_verts {
                                // pass.
                                pass.draw(vert_range(start, count), 0..1);
                            }
                            pass.cfg_pop_debug_group();
                        }
                        CommandType::Stroke { params } => {
                            counter.stroke += 1;

                            pass.cfg_push_debug_group("stroke");

                            let bg = bind_group!(self, images, cmd.image, cmd.alpha_mask);
                            {
                                pass.set_pipeline(states.stroke());
                                pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                                uniforms_offset += std::mem::size_of::<Params>() as u32;
                            }

                            let _ = pass.set_vertex_value(0, &view_size);

                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.stroke_verts {
                                    pass.draw(vert_range(start, count), 0..1);
                                }
                            }

                            pass.cfg_pop_debug_group();
                        }
                        CommandType::StencilStroke { params1, params2 } => {
                            counter.stencil_stroke += 1;

                            pass.cfg_push_debug_group("stencil stroke");
                            let s = states.stencil_stroke();
                            let bg = bind_group!(self, images, cmd.image, cmd.alpha_mask);
                            // pipeline state + stroke_shape_stencil_state

                            // stroke base
                            {
                                pass.set_pipeline(s.stroke_base());
                                pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                                uniforms_offset += std::mem::size_of::<Params>() as u32;
                            }

                            let _ = pass.set_vertex_value(0, &view_size);
                            // uniforms_offset += pass.set_fragment_value(uniforms_offset, params1);

                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.stroke_verts {
                                    // encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64)
                                    pass.draw(vert_range(start, count), 0..1);
                                }
                            }
                            // todo:
                            // draw antialiased pixels
                            // let bg = bind_group!(self, images, cmd.image, cmd.alpha_mask);
                            {
                                pass.set_pipeline(s.aa_pixels());
                                pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                                uniforms_offset += std::mem::size_of::<Params>() as u32;
                            }

                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.stroke_verts {
                                    pass.draw(vert_range(start, count), 0..1);
                                }
                            }

                            // clear stencil buffer
                            {
                                pass.set_pipeline(s.clear_stencil());
                                // pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                                // uniforms_offset += std::mem::size_of::<Params>() as u32;
                            }

                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.stroke_verts {
                                    pass.draw(vert_range(start, count), 0..1);
                                }
                            }

                            pass.cfg_pop_debug_group();
                        }
                        CommandType::Triangles { .. } => {
                            counter.triangles += 1;

                            pass.cfg_push_debug_group("triangles");

                            let bg = bind_group!(self, images, cmd.image, cmd.alpha_mask);
                            pass.set_pipeline(states.triangles());
                            let _ = pass.set_vertex_value(0, &view_size);

                            // uniforms_offset += pass.set_fragment_value(uniforms_offset, params);
                            pass.set_bind_group(0, bg.as_ref(), &[uniforms_offset]);
                            uniforms_offset += std::mem::size_of::<Params>() as u32;

                            // pass.set_bind_group(index, bind_group, offsets)
                            if let Some((start, count)) = cmd.triangles_verts {
                                // encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, start as u64, count as u64);
                                // pass.draw()
                                pass.draw(vert_range(start, count), 0..1);
                            }
                            pass.cfg_pop_debug_group();
                        }
                        CommandType::ClearRect {
                            x,
                            y,
                            width,
                            height,
                            // color,
                            ..
                        } => {
                            counter.clear_rect += 1;

                            pass.cfg_push_debug_group("clear rect");

                            let bg = &self.clear_rect_bind_group;

                            pass.set_pipeline(states.clear_rect());
                            pass.set_bind_group(0, bg, &[clear_rect_uniform_offset]);
                            clear_rect_uniform_offset += std::mem::size_of::<ClearRect>() as u32;

                            // pass.set_scissor_rect(*x as _, *y as _, *width as _, *height as _);
                            pass.set_viewport(*x as _, *y as _, *width as _, *height as _, 0.0, 1.0);

                            pass.draw(0..4, 0..1);

                            // pass.set_scissor_rect(0, 0, view_size.w as _, view_size.h as _);
                            pass.set_viewport(0.0, 0.0, view_size.w as _, view_size.h as _, 0.0, 1.0);

                            pass.cfg_pop_debug_group();
                        }
                        CommandType::SetRenderTarget(target) => {
                            counter.set_render_target += 1;

                            if render_target != *target {
                                render_target = *target;
                                drop(pass);
                                self.ctx.queue().submit(Some(encoder.finish()));

                                should_submit = false;
                                continue 'new_pass;
                            } else {
                                continue 'continued;
                            }
                        }
                    }
                }
            }

            // todo!("uniforms_offset {:?}", uniforms_offset);

            // if there's a pending submit
            if should_submit {
                self.ctx.queue().submit(Some(encoder.finish()));
            }

            // println!("render end");
            println!("counter {:?}", counter);
        }
    }

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        let label = format!("{:?}", info);
        WGPUTexture::new(&self.ctx, info, &label)
    }

    fn update_image(&mut self, image: &mut Self::Image, src: ImageSource, x: usize, y: usize) -> Result<(), ErrorKind> {
        image.update(src, x, y)
    }

    fn delete_image(&mut self, image: Self::Image) {
        // we don't have to do anything since the textures will be freed by wgpu automatically
    }

    fn screenshot(&mut self) -> Result<ImgVec<RGBA8>, ErrorKind> {
        todo!()
    }
}

impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        Self {
            r: c.r as _,
            g: c.g as _,
            b: c.b as _,
            a: c.a as _,
        }
    }
}
