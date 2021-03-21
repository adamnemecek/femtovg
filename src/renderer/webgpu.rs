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

fn new_render_pass<'a>(
    // ctx: WGPUContext,
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    // command_buffer: &'a wgpu::CommandBuffer,
    clear_color: Color,
    stencil_texture: &mut WGPUStencilTexture,
    vertex_buffer: &WGPUVec<Vertex>,

    view_size: Size,
    // ) -> wgpu::CommandEncoder {
) {
    stencil_texture.resize(view_size);

    let pass_desc = wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: target,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color.into()),
                store: false,
            },
        }],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: stencil_texture.tex(), //&'a TextureView,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: false,
            }), //Option<Operations<f32>>,
            stencil_ops: None,                 //Option<Operations<u32>>,
        }),
    };

    // todo set cull mode on the

    // let mut encoder = ctx
    //     .device()
    //     .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let mut pass = encoder.begin_render_pass(&pass_desc);
    pass.set_viewport(0.0, 0.0, view_size.w as _, view_size.h as _, 0.0, 1.0);

    pass.set_vertex_buffer(0, vertex_buffer.slice());
    // pass.set_vertex_buffer(1, buffer_slice)
    todo!()

    // encoder.set_vertex_buffer(0, vertex_buffer.as_slice());
    // encoder

    // encoder
}

pub struct WGPUStates {}

impl WGPUStates {
    pub fn new() -> Self {
        Self {}
    }
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

    cache: WGPUPipelineCache,

    view_size: Size,
    swap_chain: WGPUSwapChain,
}

fn create_bind_group() {}

impl WGPU {
    pub fn new(device: &wgpu::Device) -> Self {
        // let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        //     label: None,
        //     source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("webgpu/shader.wgsl"))),
        //     flags: wgpu::ShaderFlags::all(),
        // });

        let default_stencil_state = 0;

        let clear_stencil_state = {
            let front = wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Keep,
            };

            let state = wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState {
                    front,
                    //todo: is default the as None?
                    back: Default::default(),
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: Default::default(),
                clamp_depth: false,
            };
        };

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
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
                    binding: 1,
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
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureViewArray(&[]),
            }],
        });

        // bind_group.destroy();

        let fill_shape_stencil_state = 0;
        let fill_anti_alias_stencil_state_nonzero = 0;
        let fill_anti_alias_stencil_state_evenodd = 0;
        let fill_stencil_state_nonzero = 0;
        let fill_stencil_state_evenodd = 0;
        let stroke_shape_stencil_state = 0;
        let stroke_anti_alias_stencil_state = 0;
        let stroke_clear_stencil_state = 0;

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Self {

        // }
        todo!();
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

    fn stencil_stroke<'a, 'b>(
        &'a mut self,
        pass: &'a mut wgpu::RenderPass<'b>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        paint1: Params,
        paint2: Params,
    ) {
        //
        // pass.set_pipeline(pipeline);
        // self.set_uniforms(pass, images, image_tex, alpha_tex)
    }

    fn triangles<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        //
        self.set_uniforms(pass, images, paint, cmd.image, cmd.alpha_mask);
        // pass.set_pipeline(pipeline)
        if let Some((start, count)) = cmd.triangles_verts {
            // pass.draw(vertices, instances)
        }
    }

    fn set_uniforms<'a>(
        &self,
        pass: &wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        paint: Params,
        image_tex: Option<ImageId>,
        alpha_tex: Option<ImageId>,
    ) {
        let tex = if let Some(id) = image_tex {
            images.get(id).unwrap()
        } else {
            &self.pseudo_texture
        };
        // pass.set_viewport(x, y, w, h, min_depth, max_depth)
    }

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

    pub fn set_target(&mut self, images: &ImageStore<WGPUTexture>, target: RenderTarget) {
        //
        if self.render_target == target {}

        let size = match target {
            RenderTarget::Screen => todo!(),
            RenderTarget::Image(id) => {
                let texture = images.get(id).unwrap();
                texture.size()
            }
        };
        self.render_target = target;
        self.view_size = size;
    }
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

fn set_uniforms<'a, 'b>(
    ctx: &WGPUContext,
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    paint: Params,
    image_tex: Option<ImageId>,
    alpha_tex: Option<ImageId>,
    pseudo_texture: &WGPUTexture,
    layout: wgpu::BindGroupLayout,
) {
    let tex = if let Some(id) = image_tex {
        images.get(id).unwrap()
    } else {
        pseudo_texture
    };

    ctx.device().create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(tex.view()),
        }],
    });
    // pass.set_tex
}

fn convex_fill<'a, 'b>(
    // &'a mut self,
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    cmd: &Command,
    paint: Params,
    vertex_buffer: &WGPUVec<Vertex>,
    index_buffer: &mut WGPUVec<u32>,
    states: &'b WGPUPipelineStates,
) {
    // encoder.push_debug_group("convex_fill");

    for drawable in &cmd.drawables {
        if let Some((start, count)) = drawable.fill_verts {
            //
            pass.set_pipeline(&states.convex_fill1());
            // pass.set_pipeline(&state.convex_fill1());

            let offset = index_buffer.len();
            let triangle_fan_index_count = index_buffer.extend_with_triange_fan_indices_cw(start as u32, count as u32);

            // encoder.begin_render_pass(desc)
            // render_pass.draw_indexed(indices, base_vertex, instances)
            // pass.set_index_buffer(buffer_slice, );
            let fmt = wgpu::IndexFormat::Uint32;
            // pass.set_index_buffer(self.index_buffer, fmt);
            pass.draw_indexed(0..0, 0, 0..0);
        }

        if let Some((start, count)) = drawable.stroke_verts {
            pass.set_pipeline(&states.convex_fill2());
            let vertex_range = start as _..(start + count) as _;
            pass.draw(vertex_range, 0..0);
        }
    }
}

fn stroke<'a, 'b>(
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    cmd: &Command,
    paint: Params,
    vertex_buffer: &WGPUVec<Vertex>,
    index_buffer: &mut WGPUVec<u32>,
    states: &'b WGPUPipelineStates,
) {

    // set_uniforms()
    //
    // draws triangle strip
    // self.set_uniforms(pass, images, paint, cmd.image, cmd.alpha_mask);
    // for drawable in &cmd.drawables {
    //     if let Some((start, count)) = drawable.stroke_verts {
    //         // pass.draw()
    //     }
    // }
}

fn stencil_stroke<'a, 'b>(
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    cmd: &Command,
    paint1: Params,
    paint2: Params,
    vertex_buffer: &WGPUVec<Vertex>,
    index_buffer: &mut WGPUVec<u32>,
    states: &'b WGPUPipelineStates,
) {
    //
    // pass.set_pipeline(pipeline);
    // self.set_uniforms(pass, images, image_tex, alpha_tex)
}

fn concave_fill<'a, 'b>(
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    cmd: &Command,
    antialias: bool,
    stencil_paint: Params,
    fill_paint: Params,
    vertex_buffer: &WGPUVec<Vertex>,
    index_buffer: &mut WGPUVec<u32>,
    states: &'b WGPUPipelineStates,
) {
    for drawable in &cmd.drawables {
        if let Some((start, count)) = drawable.fill_verts {
            let offset = index_buffer.len();
            index_buffer.extend_with_triange_fan_indices_cw(start as _, count as _);
            pass.draw_indexed(0..0, 0, 0..0);
        }
    }
    pass.set_pipeline(states.concave_fill1());
    // set_uniforms

    // fringes
    if antialias {

    }
}

fn triangles<'a, 'b>(
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    cmd: &Command,
    params: Params,
    vertex_buffer: &WGPUVec<Vertex>,
    index_buffer: &mut WGPUVec<u32>,
    states: &'b WGPUPipelineStates,
) {
}

fn clear_rect<'a, 'b>(
    pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    cmd: &Command,
    vertex_buffer: &WGPUVec<Vertex>,
    index_buffer: &mut WGPUVec<u32>,
    states: &'b WGPUPipelineStates,
) {
}

impl WGPU {}

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

        let mut encoder = self
            .ctx
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

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
        let mut i = 0;

        'outer: while i < commands.len() {
            let target_texture = match render_target {
                RenderTarget::Screen => {
                    self.swap_chain.get_current_frame().unwrap()

                    // println!("render target: screen");
                    // let d = self.layer.next_drawable().unwrap().to_owned();
                    // let tex = d.texture().to_owned();
                    // drawable = Some(d);
                    // tex
                }
                RenderTarget::Image(id) => {
                    // println!("render target: image: {:?}", id);
                    // images.get(id).unwrap()
                    todo!();
                }
            };
            let pass_desc = new_pass_descriptor();
            let mut pass = encoder.begin_render_pass(&pass_desc);

            // pass.set_bind_group(index, bind_group, offsets)

            // encoder.begin_render_pass(desc)

            // pass.set_viewport(x, y, w, h, min_depth, max_depth)

            // let mut state = None;
            while i < commands.len() {
                let cmd = &commands[i];
                i += 1;

                let blend: WGPUBlend = cmd.composite_operation.into();
                let states = if let Some(prev_states) = prev_states {
                    if prev_states.matches(blend, texture_format) {
                        prev_states
                    } else {
                        self.cache.get(blend, texture_format)
                    }
                } else {
                    self.cache.get(blend, texture_format)
                };
                prev_states = Some(states);

                match &cmd.cmd_type {
                    CommandType::ConvexFill { params } => {
                        convex_fill(
                            &mut pass,
                            images,
                            cmd,
                            *params,
                            &self.vertex_buffer,
                            &mut self.index_buffer,
                            states,
                        );
                    }
                    CommandType::ConcaveFill {
                        stencil_params,
                        fill_params,
                    } => {
                        concave_fill(
                            &mut pass,
                            images,
                            cmd,
                            self.antialias,
                            *stencil_params,
                            *fill_params,
                            &self.vertex_buffer,
                            &mut self.index_buffer,
                            states,
                        );
                    }
                    CommandType::Stroke { params } => {
                        stroke(
                            &mut pass,
                            images,
                            cmd,
                            *params,
                            &self.vertex_buffer,
                            &mut self.index_buffer,
                            states,
                        );
                    }
                    CommandType::StencilStroke { params1, params2 } => {
                        stencil_stroke(
                            &mut pass,
                            images,
                            cmd,
                            *params1,
                            *params2,
                            &self.vertex_buffer,
                            &mut self.index_buffer,
                            states,
                        );
                    }
                    CommandType::Triangles { params } => {
                        triangles(
                            &mut pass,
                            images,
                            cmd,
                            *params,
                            &self.vertex_buffer,
                            &mut self.index_buffer,
                            states,
                        );
                    }
                    CommandType::ClearRect {
                        x,
                        y,
                        width,
                        height,
                        color,
                    } => {
                        clear_rect(
                            &mut pass,
                            images,
                            cmd,
                            // *params,
                            &self.vertex_buffer,
                            &mut self.index_buffer,
                            states,
                        );
                    }
                    CommandType::SetRenderTarget(target) => {
                        render_target = *target;
                        // let buffer = encoder.finish();
                        // self.ctx.queue().submit(Some(buffer));
                        // pass = encoder.begin_render_pass(&pass_desc);
                        continue 'outer;
                    }
                }
            }
        }

        let buffer = encoder.finish();
        self.ctx.queue().submit(Some(buffer));
    }

    fn alloc_image(&mut self, info: ImageInfo) -> Result<Self::Image, ErrorKind> {
        todo!()
    }

    fn update_image(
        &mut self,
        image: &mut Self::Image,
        data: ImageSource,
        x: usize,
        y: usize,
    ) -> Result<(), ErrorKind> {
        todo!()
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
