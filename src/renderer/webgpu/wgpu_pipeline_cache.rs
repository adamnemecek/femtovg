// use fnv::FnvHashMap;
use super::{
    WGPUBlend,
    WGPUContext,
};
use std::{
    cell::UnsafeCell,
    rc::Rc,
};
use std::{
    collections::HashMap,
    pin::Pin,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PipelineCacheKey {
    pub blend_func: WGPUBlend,
    pub texture_format: wgpu::TextureFormat,
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
    depth_stencil: Option<wgpu::DepthStencilState>,
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
        depth_stencil,
        multisample: wgpu::MultisampleState::default(),
    })
}

// fn clear_stencil_state(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
//     wgpu::DepthStencilState {
//         format,
//         depth_write_enabled: false,
//         depth_compare: wgpu::CompareFunction::Less,
//         stencil: wgpu::StencilState {
//             front: wgpu::StencilFaceState {
//                 compare: wgpu::CompareFunction::Less,
//                 fail_op: wgpu::StencilOperation::Keep,
//                 depth_fail_op: wgpu::StencilOperation::Keep,
//                 pass_op: wgpu::StencilOperation::Keep,
//             },
//             back: wgpu::StencilFaceState {
//                 compare: wgpu::CompareFunction::Less,
//                 fail_op: wgpu::StencilOperation::Keep,
//                 depth_fail_op: wgpu::StencilOperation::Keep,
//                 pass_op: wgpu::StencilOperation::Keep,
//             },
//             read_mask: 0xff,
//             write_mask: 0xff,
//         },
//         bias: wgpu::DepthBiasState::default(),
//         clamp_depth: false,
//     }
// }

fn fill_shape_stencil_state(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                pass_op: wgpu::StencilOperation::IncrementWrap,
                ..Default::default()
            },
            back: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                pass_op: wgpu::StencilOperation::DecrementWrap,
                ..Default::default()
            },
            read_mask: 0xff,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn fill_anti_alias_stencil_state_nonzero(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Equal,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Keep,
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0xff,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn fill_anti_alias_stencil_state_evenodd(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Equal,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Keep,
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0x1,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn fill_stencil_state_nonzero(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::NotEqual,
                fail_op: wgpu::StencilOperation::Zero,
                depth_fail_op: wgpu::StencilOperation::Zero,
                pass_op: wgpu::StencilOperation::Zero,
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0xff,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn fill_stencil_state_evenodd(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::NotEqual,
                fail_op: wgpu::StencilOperation::Zero,
                depth_fail_op: wgpu::StencilOperation::Zero,
                pass_op: wgpu::StencilOperation::Zero,
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0x1,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn stroke_shape_stencil_state(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Equal,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::IncrementClamp,
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0xff,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn stroke_anti_alias_stencil_state(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                // compare: wgpu::CompareFunction::NotEqual,
                // fail_op: wgpu::StencilOperation::Zero,
                // depth_fail_op: wgpu::StencilOperation::Zero,
                pass_op: wgpu::StencilOperation::Keep,
                ..Default::default()
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0xff,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

fn stroke_clear_stencil_state(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                fail_op: wgpu::StencilOperation::Zero,
                depth_fail_op: wgpu::StencilOperation::Zero,
                pass_op: wgpu::StencilOperation::Zero,
            },
            back: wgpu::StencilFaceState::default(),
            read_mask: 0xff,
            write_mask: 0xff,
        },
        bias: wgpu::DepthBiasState::default(),
        clamp_depth: false,
    }
}

pub struct WGPUPipelineStates {
    blend_func: WGPUBlend,
    texture_format: wgpu::TextureFormat,
    convex_fill1: wgpu::RenderPipeline,
    convex_fill2: wgpu::RenderPipeline,
}

impl WGPUPipelineStates {
    pub fn matches(&self, blend_func: WGPUBlend, format: wgpu::TextureFormat) -> bool {
        self.blend_func == blend_func && self.texture_format == format
    }

    pub fn blend_func(&self) -> WGPUBlend {
        self.blend_func
    }

    pub fn convex_fill1(&self) -> &wgpu::RenderPipeline {
        &self.convex_fill1
    }

    pub fn convex_fill2(&self) -> &wgpu::RenderPipeline {
        &self.convex_fill2
    }

    pub fn new(
        ctx: &WGPUContext,
        blend_func: WGPUBlend,
        format: wgpu::TextureFormat,
        shader: &wgpu::ShaderModule,
        // vertex_desc: &wgpu::VertexBufferLayout,
    ) -> Self {
        let antialias = create_pipeline(
            ctx,
            None,
            shader,
            format,
            wgpu::PrimitiveTopology::TriangleStrip,
            wgpu::FrontFace::Ccw,
            None,
            None,
        );
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
    // inner: std::rc::Rc<std::cell::RefCell<HashMap<PipelineCacheKey, WGPUPipelineState>>>,
    inner: std::cell::UnsafeCell<HashMap<PipelineCacheKey, WGPUPipelineStates>>,
    ctx: WGPUContext,
    // ph: &'a std::marker::PhantomData<()>,
}

impl WGPUPipelineCache {
    pub fn new(
        ctx: &WGPUContext,
        shader: wgpu::ShaderModule, // vert: &wgpu::
    ) -> Self {
        Self {
            shader,
            inner: Default::default(),
            ctx: ctx.clone(),
        }
    }

    pub fn get<'a>(&'a self, blend_func: WGPUBlend, texture_format: wgpu::TextureFormat) -> &'a WGPUPipelineStates {
        let key = PipelineCacheKey {
            blend_func,
            texture_format,
        };
        let r = unsafe { self.inner.get().as_mut().unwrap() };

        if !r.contains_key(&key) {
            let ps = WGPUPipelineStates::new(
                &self.ctx,
                blend_func,
                texture_format,
                &self.shader,
                // crate::Vertex::desc(),
            );
            r.insert(key, ps);
            // self.inner.insert(key, Rc::new(ps));
            // self.inner.borrow_mut().insert(key, ps);
        }

        // &self.inner.borrow()[&key]
        // unsafe { &self.inner.get().as_ref().unwrap()[&key] }
        &r[&key]
        // self.inner.borrow().get(&key).as_ref().unwrap()
        // todo!()
    }
}
