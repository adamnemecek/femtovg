// use crate

use super::{
    WGPUContext,
    WGPUTexture,
    WGPUVar,
    WGPUVec,
};

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
    Params,
    Rect,
    Size,
};

fn create_bind_group(
    ctx: &WGPUContext,
    // pass: &'a mut wgpu::RenderPass<'b>,
    images: &ImageStore<WGPUTexture>,
    view_size: WGPUVar<Size>,
    uniforms: WGPUVar<Params>,
    image_tex: Option<ImageId>,
    alpha_tex: Option<ImageId>,
    pseudo_tex: &WGPUTexture,

    layout: wgpu::BindGroupLayout,
    // out: &mut wgpu::BindGroup,
) -> wgpu::BindGroup {
    let tex = if let Some(id) = image_tex {
        images.get(id).unwrap()
    } else {
        pseudo_tex
    };

    let alpha_tex = if let Some(id) = alpha_tex {
        images.get(id).unwrap()
    } else {
        pseudo_tex
    };

    ctx.device().create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &[
            //viewsize
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: view_size.as_ref(),
                    offset: 0,
                    size: None,
                },
            },
            //uniforms
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer {
                    buffer: uniforms.as_ref(),
                    offset: 0,
                    size: None,
                },
            },
            // texture
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(tex.view()),
            },
            // sampler
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(tex.sampler()),
            },
            // alpha texture
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::TextureView(alpha_tex.view()),
            },
            // alpha sampler
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Sampler(alpha_tex.sampler()),
            },
        ],
    })
    // pass.set_tex
}

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    inner: u32,
}
pub struct WGPUBindGroupCache {
    // arena: generational_arena::Arena<wgpu::BindGroup>,
    inner: HashMap<CacheKey, wgpu::BindGroup>,
}

impl WGPUBindGroupCache {
    pub fn get(&self) -> &wgpu::BindGroup {
        todo!()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}
