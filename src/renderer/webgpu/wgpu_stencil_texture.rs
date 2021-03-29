use wgpu::TextureFormat;

use super::WGPUContext;
use crate::geometry::Size;
pub struct WGPUStencilTexture {
    //
    ctx: WGPUContext,
    size: Size,
    tex: wgpu::Texture,
    view: wgpu::TextureView,
    gen: u32,
}

fn label(gen: u32) -> String {
    format!("stencil texture {}", gen)
}

impl WGPUStencilTexture {
    pub fn new(ctx: &WGPUContext, size: Size) -> Self {
        let gen = 0;
        let label = label(gen);
        let desc = new_stencil_descriptor(size, &label);

        println!("reallocating texture");
        let tex = ctx.device().create_texture(&desc);
        let view = tex.create_view(&Default::default());
        Self {
            ctx: ctx.clone(),
            tex,
            size,
            view,
            gen,
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn resize(&mut self, size: Size) {
        if size == self.size {
            return;
        }
        println!("reallocating texture");
        // if self.size().contains(&size) {
        //     return;
        // }
        // let size = size.max(&self.size());

        self.gen += 1;
        let label = label(self.gen);
        let desc = new_stencil_descriptor(size, &label);
        self.tex.destroy();

        self.tex = self.ctx.device().create_texture(&desc);
        self.view = self.tex.create_view(&Default::default());
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

// impl Drop for WGPUStencilTexture {
//     fn drop(&mut self) {
//         self.tex.destroy()
//     }
// }

pub(crate) fn new_stencil_descriptor<'a>(size: Size, label: &'a str) -> wgpu::TextureDescriptor<'a> {
    wgpu::TextureDescriptor {
        label: Some(label),
        size: size.into(),
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        //todo!
        // format: wgpu::TextureFormat::R8Unorm,
        format: wgpu::TextureFormat::Depth24PlusStencil8,
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
    }
}
