// use fnv::FnvHashMap;
use super::{
    WGPUBlend,
    WGPUContext,
};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PipelineCacheKey {
    pub blend_func: WGPUBlend,
    pub texture_format: wgpu::TextureFormat,
}

pub struct WGPUPipelineState {
    blend_func: WGPUBlend,
    texture_format: wgpu::TextureFormat,
}

impl crate::Vertex {
    pub fn desc() -> wgpu::VertexAttribute {
        todo!()
    }
}

fn create_pipeline(
    ctx: &WGPUContext,
    label: Option<&str>,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    topology: wgpu::PrimitiveTopology,
    front_face: wgpu::FrontFace,
    cull_mode: Option<wgpu::Face>,
) -> wgpu::RenderPipeline {
    ctx.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: None,
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            //todo!
            targets: &[format.into()],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            front_face,
            cull_mode,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
    });
    todo!()
}

impl WGPUPipelineState {
    pub fn new(
        ctx: &WGPUContext,
        blend_func: WGPUBlend,
        format: wgpu::TextureFormat,
        shader: &wgpu::ShaderModule,
        // vertex_desc: &wgpu::VertexBufferLayout,
    ) -> Self {

        // let stencil = ctx.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {

        // });
        let pipeline = ctx.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit"),
            layout: None,
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                //todo!
                targets: &[format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });
        todo!()
        // Self {

        // }
    }
}

// struct
pub struct WGPUPipelineCache {
    shader: wgpu::ShaderModule,
    inner: HashMap<PipelineCacheKey, Rc<WGPUPipelineState>>,
    context: WGPUContext,
}

impl WGPUPipelineCache {
    pub fn new(
        ctx: &WGPUContext,
        shader_module: wgpu::ShaderModule, // vert: &wgpu::
    ) -> Self {
        // ctx.device().create_render_pipeline(&);

        todo!()
    }

    pub fn get(&mut self, blend_func: WGPUBlend, texture_format: wgpu::TextureFormat) -> Rc<WGPUPipelineState> {
        let key = PipelineCacheKey {
            blend_func,
            texture_format,
        };

        if !self.inner.contains_key(&key) {
            let ps = WGPUPipelineState::new(
                &self.context,
                blend_func,
                texture_format,
                &self.shader,
                // crate::Vertex::desc(),
            );
            self.inner.insert(key, Rc::new(ps));
        }

        self.inner.get(&key).unwrap().clone()
    }
}
