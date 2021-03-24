mod wgpu_vec;
use wgpu::util::RenderEncoder;
pub use wgpu_vec::*;

mod wgpu_queue;
pub use wgpu_queue::*;

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
    // images: &'a ImageStore<WGPUTexture>,
    // command_buffer: &'a wgpu::CommandBuffer,
    clear_color: Color,
    stencil_texture: &'a mut WGPUStencilTexture,
    vertex_buffer: &'a WGPUVec<Vertex>,
    index_buffer: &'a WGPUVec<u32>,
    view_size: Size,
    // ) -> wgpu::CommandEncoder {
) -> wgpu::RenderPass<'a> {
    stencil_texture.resize(view_size);

    let pass_desc = wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: target,
            resolve_target: None, // todo! what's this?
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color.into()),
                store: true,
            },
        }],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: stencil_texture.view(),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(0.0),
                store: false,
            }),
            // todo: what is this?
            stencil_ops: None, //Option<Operations<u32>>,
        }),
    };

    // todo set cull mode on the state

    // let mut encoder = ctx
    //     .device()
    //     .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let mut pass = encoder.begin_render_pass(&pass_desc);
    pass.set_viewport(0.0, 0.0, view_size.w as _, view_size.h as _, 0.0, 1.0);

    pass.set_vertex_buffer(0, vertex_buffer.slice());
    pass.set_stencil_reference(0);

    pass.set_index_buffer(index_buffer.slice(), wgpu::IndexFormat::Uint32);

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
    // default_stencil_state: wgpu::RenderPipeline,
    // fill_shape_stencil_state: wgpu::RenderPipeline,
    // fill_anti_alias_stencil_state_nonzero: wgpu::RenderPipeline,
    // fill_anti_alias_stencil_state_evenodd: wgpu::RenderPipeline,
    // fill_stencil_state_nonzero: wgpu::RenderPipeline,
    // fill_stencil_state_evenodd: wgpu::RenderPipeline,

    // stroke_shape_stencil_state: wgpu::RenderPipeline,
    // stroke_anti_alias_stencil_state: wgpu::RenderPipeline,
    // stroke_clear_stencil_state: wgpu::RenderPipeline,

    // convex_fill1: wgpu::RenderPipeline,
    // convex_fill2: wgpu::RenderPipeline,
    stencil_texture: WGPUStencilTexture,
    index_buffer: WGPUVec<u32>,
    vertex_buffer: WGPUVec<Vertex>,
    render_target: RenderTarget,
    pseudo_texture: WGPUTexture,

    pipeline_cache: WGPUPipelineCache,
    bind_group_cache: WGPUBindGroupCache,
    clear_color: Color,
    view_size: Size,
    swap_chain: WGPUSwapChain,
    bind_group_layout: wgpu::BindGroupLayout,
    // clear_rect_bind_group_layout: wgpu::BindGroupLayout,
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
            label: None,
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
                // wgpu::BindGroupLayoutEntry {
                //     binding: 1,
                //     visibility: wgpu::ShaderStage::FRAGMENT,
                //     ty: wgpu::BindingType::Buffer {
                //         ty: wgpu::BufferBindingType::Uniform,
                //         has_dynamic_offset: false,
                //         min_binding_size: None,
                //     },
                //     count: None,
                // },
                // texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
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
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
                // alpha texture
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
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
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        });

        let view_size_size: u32 = std::mem::size_of::<Size>() as _;
        let param_size: u32 = std::mem::size_of::<Params>() as _;
        let pipeline_layout = ctx.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[
                wgpu::PushConstantRange {
                    stages: wgpu::ShaderStage::VERTEX,
                    range: 0..view_size_size,
                },
                wgpu::PushConstantRange {
                    stages: wgpu::ShaderStage::FRAGMENT,
                    range: view_size_size..(view_size_size + param_size),
                },
            ],
        });

        // let clear_rect_bind_group_layout = ctx.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStage::VERTEX,
        //             ty: wgpu::BindingType::Texture {
        //                 sample_type: wgpu::TextureSampleType::Float { filterable: false },
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //                 multisampled: false,
        //             },
        //             count: None,
        //         },
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 1,
        //             visibility: wgpu::ShaderStage::VERTEX,
        //             ty: wgpu::BindingType::Texture {
        //                 sample_type: wgpu::TextureSampleType::Float { filterable: false },
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //                 multisampled: false,
        //             },
        //             count: None,
        //         },
        //     ],
        // });

        let clear_rect_pipeline_layout = ctx.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            // bind_group_layouts: &[&clear_rect_bind_group_layout],
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

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

        // let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: None,
        //     layout: &bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: wgpu::BindingResource::TextureViewArray(&[]),
        //     }],
        // });

        // bind_group.destroy();

        // let fill_shape_stencil_state = 0;
        // let fill_anti_alias_stencil_state_nonzero = 0;
        // let fill_anti_alias_stencil_state_evenodd = 0;
        // let fill_stencil_state_nonzero = 0;
        // let fill_stencil_state_evenodd = 0;
        // let stroke_shape_stencil_state = 0;
        // let stroke_anti_alias_stencil_state = 0;
        // let stroke_clear_stencil_state = 0;

        let encoder = ctx
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let stencil_texture = WGPUStencilTexture::new(ctx, view_size);
        let index_buffer = WGPUVec::new_index(ctx, 1000);
        let vertex_buffer = WGPUVec::new_vertex(ctx, 1000);

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
        // Self {

        // }
        // todo!();
        Self {
            clear_color,
            antialias: true,
            stencil_texture,
            ctx: ctx.clone(),
            index_buffer,
            vertex_buffer,
            render_target: RenderTarget::Screen,
            pseudo_texture,
            pipeline_cache,   //: Default::default(),
            bind_group_cache, //: Default::default(),
            view_size,
            bind_group_layout,
            swap_chain,
            // clear_rect_bind_group_layout,
            // swap_chain: WGPUSwapChain::new()
            // index_buffer: WGPUVec<u32>,
            // vertex_buffer: WGPUVec<Vertex>,
            // render_target: RenderTarget,
            // pseudo_texture: WGPUTexture,

            // pipeline_cache: WGPUPipelineCache,
            // bind_group_cache: WGPUBindGroupCache,

            // view_size: Size,
            // swap_chain: WGPUSwapChain,
            // bind_group_layout: wgpu::BindGroupLayout,
        }
    }

    // fn convex_fill<'a, 'b>(
    //     &'b mut self,
    //     pass: &'a mut wgpu::RenderPass<'b>,
    //     images: &ImageStore<WGPUTexture>,
    //     cmd: &Command,
    //     paint: Params,
    // ) where 'b : 'a
    // {
    //     // encoder.push_debug_group("convex_fill");

    //     for drawable in &cmd.drawables {
    //         if let Some((start, count)) = drawable.fill_verts {
    //             //
    //             pass.set_pipeline(&self.convex_fill1);

    //             let offset = self.index_buffer.len();
    //             let triangle_fan_index_count = self
    //                 .index_buffer
    //                 .extend_with_triange_fan_indices_cw(start as u32, count as u32);

    //             // encoder.begin_render_pass(desc)
    //             // render_pass.draw_indexed(indices, base_vertex, instances)
    //             // pass.set_index_buffer(buffer_slice, );
    //             let fmt = wgpu::IndexFormat::Uint32;
    //             // pass.set_index_buffer(self.index_buffer, fmt);
    //             pass.draw_indexed(0..0, 0, 0..0);
    //         }

    //         if let Some((start, count)) = drawable.stroke_verts {
    //             pass.set_pipeline(&self.convex_fill2);
    //             let vertex_range = start as _..(start + count) as _;
    //             pass.draw(vertex_range, 0..0);
    //         }
    //     }
    // }

    // fn stroke<'a>(
    //     &'a mut self,
    //     pass: &mut wgpu::RenderPass<'a>,
    //     images: &ImageStore<WGPUTexture>,
    //     cmd: &Command,
    //     paint: Params,
    // ) {
    //     //
    //     // draws triangle strip
    //     self.set_uniforms(pass, images, paint, cmd.image, cmd.alpha_mask);
    //     for drawable in &cmd.drawables {
    //         if let Some((start, count)) = drawable.stroke_verts {
    //             // pass.draw()
    //         }
    //     }
    // }

    // fn stencil_stroke<'a, 'b>(
    //     &'a mut self,
    //     pass: &'a mut wgpu::RenderPass<'b>,
    //     images: &ImageStore<WGPUTexture>,
    //     cmd: &Command,
    //     paint1: Params,
    //     paint2: Params,
    // ) {
    //     //
    //     // pass.set_pipeline(pipeline);
    //     // self.set_uniforms(pass, images, image_tex, alpha_tex)
    // }

    // fn triangles<'a>(
    //     &'a mut self,
    //     pass: &mut wgpu::RenderPass<'a>,
    //     images: &ImageStore<WGPUTexture>,
    //     cmd: &Command,
    //     paint: Params,
    // ) {
    //     //
    //     // self.set_uniforms(pass, images, paint, cmd.image, cmd.alpha_mask);
    //     // pass.set_pipeline(pipeline)
    //     if let Some((start, count)) = cmd.triangles_verts {
    //         // pass.draw(vertices, instances)
    //     }
    // }

    // fn set_uniforms<'a>(
    //     &self,
    //     pass: &wgpu::RenderPass<'a>,
    //     images: &ImageStore<WGPUTexture>,
    //     paint: Params,
    //     image_tex: Option<ImageId>,
    //     alpha_tex: Option<ImageId>,
    // ) {
    //     let tex = if let Some(id) = image_tex {
    //         images.get(id).unwrap()
    //     } else {
    //         &self.pseudo_texture
    //     };
    //     // pass.set_viewport(x, y, w, h, min_depth, max_depth)
    // }

    fn clear_rect<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) {
        //
        let ndc_rect = Rect {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        };
    }

    // pub fn set_target(&mut self, images: &ImageStore<WGPUTexture>, target: RenderTarget) {
    //     //
    //     if self.render_target == target {}

    //     let size = match target {
    //         RenderTarget::Screen => todo!(),
    //         RenderTarget::Image(id) => {
    //             let texture = images.get(id).unwrap();
    //             texture.size()
    //         }
    //     };
    //     self.render_target = target;
    //     self.view_size = size;
    // }
}

fn new_pass<'a>() -> wgpu::RenderPass<'a> {
    todo!()
}

fn new_pass_descriptor<'a, 'b>() -> wgpu::RenderPassDescriptor<'a, 'b> {
    todo!()
}

pub struct TextureBindings {
    // tex_tex:
}

// fn convex_fill<'a, 'b>(
//     // &'a mut self,
//     pass: &'a mut wgpu::RenderPass<'b>,
//     images: &ImageStore<WGPUTexture>,
//     cmd: &Command,
//     paint: Params,
//     vertex_buffer: &WGPUVec<Vertex>,
//     index_buffer: &mut WGPUVec<u32>,
//     states: &'b WGPUPipelineStates,
// ) {
//     // encoder.push_debug_group("convex_fill");

//     for drawable in &cmd.drawables {
//         if let Some((start, count)) = drawable.fill_verts {
//             //
//             pass.set_pipeline(&states.convex_fill1());

//             // pass.set_pipeline(&state.convex_fill1());

//             let offset = index_buffer.len();
//             let triangle_fan_index_count = index_buffer.extend_with_triange_fan_indices_cw(start as u32, count as u32);

//             // encoder.begin_render_pass(desc)
//             // render_pass.draw_indexed(indices, base_vertex, instances)
//             // pass.set_index_buffer(buffer_slice, );
//             let fmt = wgpu::IndexFormat::Uint32;
//             // pass.set_index_buffer(self.index_buffer, fmt);
//             pass.draw_indexed(0..0, 0, 0..0);
//         }

//         if let Some((start, count)) = drawable.stroke_verts {
//             pass.set_pipeline(&states.convex_fill2());
//             let vertex_range = start as _..(start + count) as _;
//             pass.draw(vertex_range, 0..0);
//         }
//     }
// }

// fn stroke<'a, 'b>(
//     ctx: &WGPUContext,
//     pass: &'a mut wgpu::RenderPass<'a>,
//     images: &ImageStore<WGPUTexture>,
//     view_size: WGPUVar<Size>,
//     cmd: &Command,
//     uniforms: WGPUVar<Params>,
//     vertex_buffer: &WGPUVec<Vertex>,
//     index_buffer: &mut WGPUVec<u32>,
//     image_tex: Option<ImageId>,
//     alpha_tex: Option<ImageId>,
//     pseudo_tex: &WGPUTexture,
//     bind_group_layout: wgpu::BindGroupLayout,
//     states: &'b WGPUPipelineStates,

//     // cache: &'a BindingGroupCache
//     bind_groups: &'a mut Vec<wgpu::BindGroup>,
// ) {
//     // set_uniforms()
//     //di
//     // draws triangle strip
//     let bind_group = create_bind_group(
//         ctx,
//         images,
//         view_size,
//         uniforms,
//         image_tex,
//         alpha_tex,
//         pseudo_tex,
//         bind_group_layout,
//     );

//     // let bind = cache.get();
//     // pass.set_bind_group(0, bind, &[]);
//     bind_groups.push(bind_group);
//     let bind_group = bind_groups.last().unwrap();
//     pass.set_bind_group(0, bind_group, &[]);

//     // pass.set_pipeline(pipeline);
//     // pass.set_bind_group(0, &bind_group, &[]);
//     for drawable in &cmd.drawables {
//         if let Some((start, count)) = drawable.stroke_verts {
//             // pass.draw()
//         }
//     }
// }

// fn stencil_stroke<'a, 'b>(
//     pass: &'a mut wgpu::RenderPass<'b>,
//     images: &ImageStore<WGPUTexture>,
//     cmd: &Command,
//     paint1: Params,
//     paint2: Params,
//     vertex_buffer: &WGPUVec<Vertex>,
//     index_buffer: &mut WGPUVec<u32>,
//     states: &'b WGPUPipelineStates,
// ) {
//     // pass.set_pipeline()
//     //
//     // pass.set_pipeline(pipeline);
//     // self.set_uniforms(pass, images, image_tex, alpha_tex)

//     // pass.set_pipeline();
// }

// fn concave_fill<'a, 'b>(
//     pass: &'a mut wgpu::RenderPass<'b>,
//     images: &ImageStore<WGPUTexture>,
//     cmd: &Command,
//     antialias: bool,
//     stencil_paint: Params,
//     fill_paint: Params,
//     vertex_buffer: &WGPUVec<Vertex>,
//     index_buffer: &mut WGPUVec<u32>,
//     states: &'b WGPUPipelineStates,
// ) {
//     for drawable in &cmd.drawables {
//         if let Some((start, count)) = drawable.fill_verts {
//             let offset = index_buffer.len();
//             index_buffer.extend_with_triange_fan_indices_cw(start as _, count as _);
//             pass.draw_indexed(0..0, 0, 0..0);
//             // pass.set_push_constants(stages, offset, data)p
//         }
//     }
//     pass.set_pipeline(states.concave_fill1());
//     // set_uniforms

//     // fringes
//     if antialias {
//         match cmd.fill_rule {
//             FillRule::NonZero => {
//                 pass.set_pipeline(states.fill_anti_alias_stencil_state_nonzero());
//             }
//             FillRule::EvenOdd => {
//                 pass.set_pipeline(states.fill_anti_alias_stencil_state_evenodd());
//             }
//         }

//         for drawable in &cmd.drawables {
//             if let Some((start, count)) = drawable.stroke_verts {
//                 // pass.draw(vertices, instances)
//             }
//         }
//     }

//     // todo: can be moved into the if statement
//     match cmd.fill_rule {
//         FillRule::NonZero => {
//             pass.set_pipeline(states.fill_anti_alias_stencil_state_nonzero());
//         }
//         FillRule::EvenOdd => {
//             pass.set_pipeline(states.fill_anti_alias_stencil_state_evenodd());
//         }
//     }

//     if let Some((start, count)) = cmd.triangles_verts {
//         // pass.
//     }
//     // pass.set_pipeline(pipeline)
// }

// fn triangles<'a, 'b>(
//     pass: &'a mut wgpu::RenderPass<'b>,
//     images: &ImageStore<WGPUTexture>,
//     cmd: &Command,
//     params: Params,
//     vertex_buffer: &WGPUVec<Vertex>,
//     index_buffer: &mut WGPUVec<u32>,
//     states: &'b WGPUPipelineStates,
// ) {
// }

// fn clear_rect<'a, 'b>(
//     pass: &'a mut wgpu::RenderPass<'b>,
//     images: &ImageStore<WGPUTexture>,
//     cmd: &Command,
//     vertex_buffer: &WGPUVec<Vertex>,
//     index_buffer: &mut WGPUVec<u32>,
//     states: &'b WGPUPipelineStates,
// ) {
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

enum TargetTexture<'a> {
    Frame(wgpu::SwapChainFrame),
    View(&'a wgpu::TextureView),
}
impl<'a> TargetTexture<'a> {
    pub fn view(&'a self) -> &'a wgpu::TextureView {
        match self {
            Self::Frame(f) => &f.output.view,
            Self::View(r) => r,
        }
    }
}

impl<'a> Drop for TargetTexture<'a> {
    fn drop(&mut self) {
        match self {
            Self::Frame(_) => println!("dropping frame"),
            Self::View(_) => println!("dropping view"),
        }
    }
}

impl Renderer for WGPU {
    type Image = WGPUTexture;
    fn set_size(&mut self, width: u32, height: u32, dpi: f32) {
        let size = Size::new(width as f32, height as f32);
        self.view_size = size;
    }

    fn render(&mut self, images: &ImageStore<Self::Image>, verts: &[Vertex], commands: &[Command]) {
        self.vertex_buffer.clear();
        self.vertex_buffer.extend_from_slice(verts);

        self.index_buffer.clear();
        self.index_buffer.resize(verts.len() * 3);

        // let texture_format = &self.swap_chain.format();
        // let format = texture_format.clone();
        let texture_format = self.swap_chain.format();

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

        // let mut current_frame = None;

        // process indices
        for cmd in commands.iter() {
            match cmd.cmd_type {
                CommandType::ConvexFill { .. } => {
                    for drawable in &cmd.drawables {
                        if let Some((start, count)) = drawable.fill_verts {
                            self.index_buffer
                                .extend_with_triange_fan_indices_cw(start as _, count as _);
                        }
                    }
                }
                CommandType::ConcaveFill { .. } => {
                    for drawable in &cmd.drawables {
                        if let Some((start, count)) = drawable.fill_verts {
                            // let offset = self.index_buffer.len();
                            self.index_buffer
                                .extend_with_triange_fan_indices_cw(start as _, count as _);
                        }
                    }
                }
                CommandType::Stroke { .. } => {}
                CommandType::StencilStroke { .. } => {}
                CommandType::Triangles { .. } => {}
                CommandType::ClearRect { .. } => {}
                CommandType::SetRenderTarget(_) => {}
            }
        }

        let mut i = 0;

        'outer: while i < commands.len() {
            let mut encoder = self.ctx.create_command_encoder(None);
            {
                // let target_texture_view = match render_target {
                //     RenderTarget::Screen => {
                //         if let Ok(frame) = self.swap_chain.get_current_frame() {
                //             TargetTexture::Frame(frame)
                //         } else {
                //             todo!()
                //         }
                //     }
                //     RenderTarget::Image(id) => {
                //         TargetTexture::View(images.get(id).unwrap().view()),
                //     }
                // };
                let frame = self.swap_chain.get_current_frame().unwrap();

                let mut pass = begin_render_pass(
                    &mut encoder,
                    // target_texture_view.view(),
                    &frame.output.view,
                    self.clear_color,
                    &mut self.stencil_texture,
                    &self.vertex_buffer,
                    &self.index_buffer,
                    self.view_size,
                );

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
                    ($self_: ident, $images: ident, $cmd: ident) => {
                        $self_.bind_group_cache.get(
                            &$self_.ctx,
                            $images,
                            &$self_.bind_group_layout,
                            $cmd.image,
                            $cmd.alpha_mask,
                            &$self_.pseudo_texture,
                        );
                    };
                }

                while i < commands.len() {
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
                        prev_states = Some(states);
                        states
                    };

                    // pass.set_push_constants(wgpu::ShaderStage::FRAGMENT, 0, &[]);

                    // uniforms_offset += std::mem::size_of::<Params>();

                    match &cmd.cmd_type {
                        CommandType::ConvexFill { params } => {
                            pass.push_debug_group("convex fill");
                            // set_uniforms
                            let s = states.convex_fill();

                            // let bg = self.bind_group_for(images, cmd.image, cmd.alpha_mask);
                            let bg = bind_group!(self, images, cmd);

                            pass.set_pipeline(s.fill_buffer());
                            pass.set_bind_group(0, bg.as_ref(), &[]);
                            uniforms_offset += pass.set_fragment_value(uniforms_offset, params);

                            for drawable in &cmd.drawables {
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
                                    let start = (start - 2) * 3;
                                    let count = (count - 2) * 3;
                                    pass.draw_indexed(vert_range(start, count), 0, 0..1);
                                    // offset += (count as u32 - 2) * 3;
                                }
                                // draw fringes

                                if let Some((start, count)) = drawable.stroke_verts {
                                    pass.set_pipeline(s.stroke_buffer());
                                    pass.draw(vert_range(start, count), 0..1);
                                }
                            }
                            pass.pop_debug_group();
                        }
                        CommandType::ConcaveFill {
                            stencil_params,
                            fill_params,
                        } => {
                            pass.push_debug_group("concave fill");
                            let s = states.concave_fill();
                            // let bg = self.bind_group_for(images, cmd.image, cmd.alpha_mask);
                            let bg = bind_group!(self, images, cmd);

                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.fill_verts {
                                    let offset = self.index_buffer.len();
                                    // self.index_buffer
                                    // .extend_with_triange_fan_indices_cw(start as _, count as _);
                                    pass.draw_indexed(0..0, 0, 0..0);
                                    // pass.set_push_constants(stages, offset, data)p
                                }
                            }
                            // pass.set_pipeline(states.concave_fill1());
                            // set_uniforms

                            // fringes
                            if self.antialias {
                                match cmd.fill_rule {
                                    FillRule::NonZero => {
                                        pass.set_pipeline(s.fringes_nonzero());
                                        // pass.set_pipeline(states.fill_anti_alias_stencil_state_nonzero());
                                    }
                                    FillRule::EvenOdd => {
                                        pass.set_pipeline(s.fringes_evenodd());
                                        // pass.set_pipeline(states.fill_anti_alias_stencil_state_evenodd());
                                    }
                                }

                                for drawable in &cmd.drawables {
                                    if let Some((start, count)) = drawable.stroke_verts {
                                        pass.draw(vert_range(start, count), 0..1);
                                        // pass.draw(vertices, instances)
                                    }
                                }
                            }

                            // todo: can be moved into the if statement below?
                            match cmd.fill_rule {
                                FillRule::NonZero => {
                                    pass.set_pipeline(s.triangle_verts_nonzero());
                                    // pass.set_pipeline(states.fill_anti_alias_stencil_state_nonzero());
                                }
                                FillRule::EvenOdd => {
                                    pass.set_pipeline(s.triangle_verts_evenodd());
                                    // pass.set_pipeline(states.fill_anti_alias_stencil_state_evenodd());
                                }
                            }

                            if let Some((start, count)) = cmd.triangles_verts {
                                // pass.
                                pass.draw(vert_range(start, count), 0..1);
                            }
                            pass.pop_debug_group();
                        }
                        CommandType::Stroke { params } => {
                            pass.push_debug_group("stroke");
                            // let bg = self.bind_group_for(images, cmd.image, cmd.alpha_mask);
                            let bg = bind_group!(self, images, cmd);

                            // pass.set_pipeline()
                            pass.set_bind_group(0, bg.as_ref(), &[]);
                            let _ = pass.set_vertex_value(0, params);
                            // pass.set_bind_group(0, bg.as_ref(), &[]);
                            uniforms_offset += pass.set_fragment_value(uniforms_offset, params);

                            // self.set_uniforms(pass, images, paint, cmd.image, cmd.alpha_mask);
                            //     for drawable in &cmd.drawables {
                            //         if let Some((start, count)) = drawable.stroke_verts {
                            //             // pass.draw()
                            //         }
                            //     }
                            pass.pop_debug_group();
                        }
                        CommandType::StencilStroke { params1, params2 } => {
                            pass.push_debug_group("stencil stroke");
                            // pipeline state + stroke_shape_stencil_state
                            let bg = bind_group!(self, images, cmd);
                            uniforms_offset += pass.set_fragment_value(uniforms_offset, params1);

                            for drawable in &cmd.drawables {
                                if let Some((start, count)) = drawable.stroke_verts {
                                    // encoder.draw_primitives(metal::MTLPrimitiveType::TriangleStrip, start as u64, count as u64)
                                    pass.draw(vert_range(start, count), 0..0);
                                }
                            }

                            // let bg = bind_group!(self, cmd);
                            uniforms_offset += pass.set_fragment_value(uniforms_offset, params1);
                            pass.pop_debug_group();
                        }
                        CommandType::Triangles { params } => {
                            pass.push_debug_group("triangles");
                            let bg = bind_group!(self, images, cmd);
                            uniforms_offset += pass.set_fragment_value(uniforms_offset, params);

                            // pass.set_bind_group(index, bind_group, offsets)
                            if let Some((start, count)) = cmd.triangles_verts {
                                // encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, start as u64, count as u64);
                                // pass.draw()
                                pass.draw(vert_range(start, count), 0..1);
                            }
                            pass.pop_debug_group();
                        }
                        CommandType::ClearRect {
                            x,
                            y,
                            width,
                            height,
                            color,
                        } => {
                            pass.push_debug_group("clear rect");
                            let ndc_rect = Rect {
                                x: -1.0,
                                y: -1.0,
                                w: 2.0,
                                h: 2.0,
                            };

                            let clear_rect = ClearRect {
                                rect: ndc_rect,
                                color: *color,
                            };

                            let bg = bind_group!(self, images, cmd);

                            pass.set_pipeline(states.clear_rect());

                            pass.set_scissor_rect(*x as _, *y as _, *width as _, *height as _);
                            pass.set_vertex_value(0, &clear_rect);
                            // pass.draw()

                            pass.pop_debug_group();
                            // clear_rect(
                            //     &mut pass,
                            //     images,
                            //     cmd,
                            //     // *params,
                            //     &self.vertex_buffer,
                            //     &mut self.index_buffer,
                            //     states,
                            // );
                        }
                        CommandType::SetRenderTarget(target) => {
                            render_target = *target;
                            continue 'outer;
                        }
                    }
                }
            }

            self.ctx.queue().submit(Some(encoder.finish()));
        }
    }

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        WGPUTexture::new(&self.ctx, info)
    }

    fn update_image(&mut self, image: &mut Self::Image, src: ImageSource, x: usize, y: usize) -> Result<(), ErrorKind> {
        image.update(src, x, y)
    }

    fn delete_image(&mut self, image: Self::Image) {
        image.delete();
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

// pub struct RenderPass<'a> {
//     inner: wgpu::RenderPass<'a>,
// }

// impl<'a> RenderPass<'a> {
//     pub fn new() -> Self {
//         todo!()
//     }

//     pub fn set_viewport(&self) {
//         // self.inner.set_viewport(x, y, w, h, min_depth, max_depth)
//     }

//     pub fn set_fragment(&self) {
//         todo!()
//         // self.inner.set_push_constants(stages, offset, data)
//     }
// }
