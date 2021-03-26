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
    layout: &wgpu::BindGroupLayout,
    uniforms: &WGPUVec<Params>,
    // view_size: WGPUVar<Size>,
    // uniforms: WGPUVar<Params>,
    image_tex: Option<ImageId>,
    alpha_tex: Option<ImageId>,
    pseudo_tex: &WGPUTexture,
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
        layout,
        entries: &[
            //viewsize
            // wgpu::BindGroupEntry {
            //     binding: 0,
            //     resource: wgpu::BindingResource::Buffer {
            //         buffer: view_size.as_ref(),
            //         offset: 0,
            //         size: None,
            //     },
            // },
            //uniforms
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: uniforms.as_ref(),
                    offset: 0,
                    size: wgpu::BufferSize::new(std::mem::size_of::<Params>() as _),
                },
            },
            // texture
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tex.view()),
            },
            // sampler
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(tex.sampler()),
            },
            // alpha texture
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&alpha_tex.view()),
            },
            // alpha sampler
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(alpha_tex.sampler()),
            },
        ],
    })
    // pass.set_tex
}

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    image_tex: Option<ImageId>,
    alpha_tex: Option<ImageId>,
}

pub struct WGPUBindGroup {
    image_tex: Option<ImageId>,
    alpha_tex: Option<ImageId>,
    inner: wgpu::BindGroup,
}

impl WGPUBindGroup {
    pub fn matches(&self, image_tex: Option<ImageId>, alpha_tex: Option<ImageId>) -> bool {
        self.image_tex == image_tex && self.alpha_tex == alpha_tex
    }
}

impl AsRef<wgpu::BindGroup> for WGPUBindGroup {
    fn as_ref(&self) -> &wgpu::BindGroup {
        &self.inner
    }
}

pub struct WGPUBindGroupCache {
    // arena: generational_arena::Arena<wgpu::BindGroup>,
    inner: std::cell::UnsafeCell<HashMap<CacheKey, WGPUBindGroup>>,
    // inner: HashMap<CacheKey, wgpu::BindGroup>,
}

impl WGPUBindGroupCache {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn get(
        &self,
        ctx: &WGPUContext,
        // pass: &'a mut wgpu::RenderPass<'b>,
        images: &ImageStore<WGPUTexture>,
        layout: &wgpu::BindGroupLayout,
        uniforms: &WGPUVec<Params>,
        // view_size: WGPUVar<Size>,
        // uniforms: WGPUVar<Params>,
        image_tex: Option<ImageId>,
        alpha_tex: Option<ImageId>,
        pseudo_tex: &WGPUTexture,
    ) -> &WGPUBindGroup {
        let key = CacheKey { image_tex, alpha_tex };
        // let inner= self.inner.get_mut();
        let r = unsafe { self.inner.get().as_mut().unwrap() };

        // if let Some(bg) = inner.get(&key) {
        //     return bg;
        // }
        // let bind_group = create_bind_group(ctx, images, layout, image_tex, alpha_tex, pseudo_tex);
        // inner.insert(key, bind_group);
        // inner.get(&key).unwrap()

        if !r.contains_key(&key) {
            let inner = create_bind_group(ctx, images, layout, uniforms, image_tex, alpha_tex, pseudo_tex);
            r.insert(
                key,
                WGPUBindGroup {
                    inner,
                    image_tex,
                    alpha_tex,
                },
            );
        }
        &r[&key]
    }

    pub fn clear(&mut self) {
        self.inner.get_mut().clear()
        // self.inner.get_mut().clear()
    }
}
