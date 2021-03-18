mod wgpu_vec;
use wgpu::util::RenderEncoder;
pub use wgpu_vec::*;

mod wgpu_queue;
pub use wgpu_queue::*;

mod wgpu_texture;
pub use wgpu_texture::*;

mod wgpu_stencil_texture;
pub use wgpu_stencil_texture::*;

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
};

use super::{
    Command,
    CommandType,
    Params,
    RenderTarget,
    Renderer,
};

use fnv::FnvHashMap;
use imgref::ImgVec;
use rgb::RGBA8;
use std::borrow::Cow;

pub struct WGPU {
    default_stencil_state: wgpu::RenderPipeline,
    fill_shape_stencil_state: wgpu::RenderPipeline,
    fill_anti_alias_stencil_state_nonzero: wgpu::RenderPipeline,
    fill_anti_alias_stencil_state_evenodd: wgpu::RenderPipeline,
    fill_stencil_state_nonzero: wgpu::RenderPipeline,
    fill_stencil_state_evenodd: wgpu::RenderPipeline,

    stroke_shape_stencil_state: wgpu::RenderPipeline,
    stroke_anti_alias_stencil_state: wgpu::RenderPipeline,
    stroke_clear_stencil_state: wgpu::RenderPipeline,

    convex_fill1: wgpu::RenderPipeline,
    convex_fill2: wgpu::RenderPipeline,

    stencil_texture: WGPUStencilTexture,
    index_buffer: WGPUVec<u32>,
    vertex_buffer: WGPUVec<Vertex>,
    render_target: RenderTarget,
    pseudo_texture: WGPUTexture,
}

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

        let fill_shape_stencil_state = 0;
        let fill_anti_alias_stencil_state_nonzero = 0;
        let fill_anti_alias_stencil_state_evenodd = 0;
        let fill_stencil_state_nonzero = 0;
        let fill_stencil_state_evenodd = 0;
        let stroke_shape_stencil_state = 0;
        let stroke_anti_alias_stencil_state = 0;
        let stroke_clear_stencil_state = 0;

        todo!()
        // Self {

        // }
    }

    fn convex_fill<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        // encoder.push_debug_group("convex_fill");

        for drawable in &cmd.drawables {
            if let Some((start, count)) = drawable.fill_verts {
                //
                pass.set_pipeline(&self.convex_fill1);

                let offset = self.index_buffer.len();
                let triangle_fan_index_count = self
                    .index_buffer
                    .extend_with_triange_fan_indices_cw(start as u32, count as u32);

                // encoder.begin_render_pass(desc)
                // render_pass.draw_indexed(indices, base_vertex, instances)
                // pass.set_index_buffer(buffer_slice, );
                let fmt = wgpu::IndexFormat::Uint32;
                // pass.set_index_buffer(self.index_buffer, fmt);
                pass.draw_indexed(0..0, 0, 0..0);
            }

            if let Some((start, count)) = drawable.stroke_verts {
                pass.set_pipeline(&self.convex_fill2);
                let vertex_range = start as _..(start + count) as _;
                pass.draw(vertex_range, 0..0);
            }
        }
    }

    fn concave_fill<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        stencil_paint: Params,
        fill_paint: Params,
    ) {
    }

    fn stroke<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        //
    }

    fn stencil_stroke<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        paint1: Params,
        paint2: Params,
    ) {
        //
    }

    fn triangles<'a>(
        &'a mut self,
        pass: &mut wgpu::RenderPass<'a>,
        images: &ImageStore<WGPUTexture>,
        cmd: &Command,
        paint: Params,
    ) {
        //
    }

    fn set_uniforms(
        &self,
        encoder: &wgpu::CommandEncoder,
        images: &ImageStore<WGPUTexture>,
        image_tex: Option<ImageId>,
        alpha_tex: Option<ImageId>,
    ) {
    }

    fn clear_rect<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>, images: &ImageStore<WGPUTexture>) {}

    pub fn set_target(&mut self) {}
}

impl Renderer for WGPU {
    type Image = WGPUTexture;
    fn set_size(&mut self, width: u32, height: u32, dpi: f32) {
        todo!()
    }
    fn render(&mut self, images: &ImageStore<Self::Image>, verts: &[Vertex], commands: &[Command]) {
        todo!()
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
