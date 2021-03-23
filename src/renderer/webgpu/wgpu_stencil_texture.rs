use wgpu::TextureFormat;

use super::WGPUContext;
use crate::geometry::Size;
pub struct WGPUStencilTexture {
    //
    ctx: WGPUContext,
    size: Size,
    tex: wgpu::Texture,
}

impl WGPUStencilTexture {
    pub fn new(ctx: &WGPUContext, size: Size) -> Self {
        let desc = new_stencil_descriptor(size);

        let tex = ctx.device().create_texture(&desc);
        Self {
            ctx: ctx.clone(),
            tex,
            size,
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        todo!()
    }

    pub fn view(&self) -> &wgpu::TextureView {
        // self.
        todo!()
    }
}

impl Drop for WGPUStencilTexture {
    fn drop(&mut self) {
        self.tex.destroy()
    }
}

fn new_stencil_descriptor<'a>(size: Size) -> wgpu::TextureDescriptor<'a> {
    wgpu::TextureDescriptor {
        label: Some("stencil texture"),
        size: size.into(),
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth24PlusStencil8,
        //todo!
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::RENDER_ATTACHMENT,
    }
}
