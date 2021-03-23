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
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            }],
        }
    }
}

fn create_pipeline<'a>(
    ctx: &WGPUContext,
    label: impl Into<Option<&'a str>>,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    topology: wgpu::PrimitiveTopology,
    cull_mode: impl Into<Option<wgpu::Face>>,
    depth_stencil: impl Into<Option<wgpu::DepthStencilState>>,
) -> wgpu::RenderPipeline {
    ctx.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: label.into(),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[crate::Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            //todo!
            targets: &[format.into()],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: cull_mode.into(),
            ..Default::default()
        },
        depth_stencil: depth_stencil.into(),
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

pub struct ConvexFill {
    fill_buffer: wgpu::RenderPipeline,
    stroke_buffer: wgpu::RenderPipeline,
}

impl ConvexFill {
    pub fn fill_buffer(&self) -> &wgpu::RenderPipeline {
        &self.fill_buffer
    }

    pub fn stroke_buffer(&self) -> &wgpu::RenderPipeline {
        &self.stroke_buffer
    }
}

pub struct ConcaveFill {
    fill_verts: wgpu::RenderPipeline,
    fringes_nonzero: wgpu::RenderPipeline,
    fringes_evenodd: wgpu::RenderPipeline,
    triangle_verts_nonzero: wgpu::RenderPipeline,
    triangle_verts_evenodd: wgpu::RenderPipeline,
}

impl ConcaveFill {
    pub fn fill_verts(&self) -> &wgpu::RenderPipeline {
        &self.fill_verts
    }

    pub fn fringes_nonzero(&self) -> &wgpu::RenderPipeline {
        &self.fringes_nonzero
    }

    pub fn fringes_evenodd(&self) -> &wgpu::RenderPipeline {
        &self.fringes_evenodd
    }

    pub fn triangle_verts_nonzero(&self) -> &wgpu::RenderPipeline {
        &self.triangle_verts_nonzero
    }

    pub fn triangle_verts_evenodd(&self) -> &wgpu::RenderPipeline {
        &self.triangle_verts_evenodd
    }
}

pub struct StencilStroke {
    stroke_base: wgpu::RenderPipeline,
    pixels: wgpu::RenderPipeline,
    clear_stencil: wgpu::RenderPipeline,
}

impl StencilStroke {
    pub fn stroke_base(&self) -> &wgpu::RenderPipeline {
        &self.stroke_base
    }

    pub fn pixels(&self) -> &wgpu::RenderPipeline {
        &self.pixels
    }

    pub fn clear_stencil(&self) -> &wgpu::RenderPipeline {
        &self.clear_stencil
    }
}

pub struct WGPUPipelineStates {
    blend_func: WGPUBlend,
    texture_format: wgpu::TextureFormat,

    convex_fill: ConvexFill,
    concave_fill: ConcaveFill,
    stroke: wgpu::RenderPipeline,
    stencil_stroke: StencilStroke,
    triangles: wgpu::RenderPipeline,
    // convex_fill1: wgpu::RenderPipeline,
    // convex_fill2: wgpu::RenderPipeline,
    // concave_fill1: wgpu::RenderPipeline,
    // concave_fill2: wgpu::RenderPipeline,
    // fill_anti_alias_stencil_state_nonzero: wgpu::RenderPipeline,
    // fill_anti_alias_stencil_state_evenodd: wgpu::RenderPipeline,
    // stroke_shape_stencil_state: wgpu::RenderPipeline,
}

impl WGPUPipelineStates {
    pub fn matches(&self, blend_func: WGPUBlend, format: wgpu::TextureFormat) -> bool {
        self.blend_func == blend_func && self.texture_format == format
    }

    pub fn blend_func(&self) -> WGPUBlend {
        self.blend_func
    }

    // pub fn stroke_shape_stencil_state(&self) -> &wgpu::RenderPipeline {
    //     &self.stroke_shape_stencil_state
    // }

    // pub fn fill_anti_alias_stencil_state_evenodd(&self) -> &wgpu::RenderPipeline {
    //     &self.fill_anti_alias_stencil_state_evenodd
    // }

    // pub fn fill_anti_alias_stencil_state_nonzero(&self) -> &wgpu::RenderPipeline {
    //     &self.fill_anti_alias_stencil_state_nonzero
    // }

    pub fn convex_fill(&self) -> &ConvexFill {
        &self.convex_fill
    }

    // pub fn convex_fill1(&self) -> &wgpu::RenderPipeline {
    //     &self.convex_fill1
    // }

    // pub fn convex_fill2(&self) -> &wgpu::RenderPipeline {
    //     &self.convex_fill2
    // }

    // pub fn concave_fill1(&self) -> &wgpu::RenderPipeline {
    //     &self.concave_fill1
    // }

    // pub fn concave_fill2(&self) -> &wgpu::RenderPipeline {
    //     &self.concave_fill2
    // }

    pub fn new(
        ctx: &WGPUContext,
        layout: &wgpu::PipelineLayout,
        blend_func: WGPUBlend,
        format: wgpu::TextureFormat,
        shader: &wgpu::ShaderModule,
        // vertex_desc: &wgpu::VertexBufferLayout,
    ) -> Self {
        // let convex_fill_stroke_buffer = create_pipeline(
        //     ctx,
        //     Some("convex_fill/stroke_buffer"),
        //     shader,
        //     format,
        //     wgpu::PrimitiveTopology::TriangleList,
        //     wgpu::FrontFace::Ccw,
        //     None,
        //     None,
        // );
        let convex_fill = ConvexFill {
            fill_buffer: create_pipeline(
                ctx,
                "convex_fill/fill_buffer",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                None,
            ),
            stroke_buffer: create_pipeline(
                ctx,
                "convex_fill/stroke_buffer",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::Face::Back,
                None,
            ),
        };

        let concave_fill = ConcaveFill {
            // stencil only pipeline state
            fill_verts: create_pipeline(
                ctx,
                "concave_fill/fill_verts",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                None,
                fill_shape_stencil_state(format),
            ),
            fringes_nonzero: create_pipeline(
                ctx,
                "concave_fill/fringes_nonzero",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                fill_anti_alias_stencil_state_nonzero(format),
            ),
            fringes_evenodd: create_pipeline(
                ctx,
                "concave_fill/fringes_evenodd",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                fill_anti_alias_stencil_state_evenodd(format),
            ),
            triangle_verts_nonzero: create_pipeline(
                ctx,
                "concave_fill/triangle_verts_nonzero",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                None,
            ),
            triangle_verts_evenodd: create_pipeline(
                ctx,
                "concave_fill/triangle_verts_evenodd",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                None,
            ),
        };

        let stroke = create_pipeline(
            ctx,
            "stroke",
            layout,
            shader,
            format,
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::Face::Back,
            None,
        );

        let stencil_stroke = StencilStroke {
            stroke_base: create_pipeline(
                ctx,
                "stroke_base",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                Some(stroke_shape_stencil_state(format)),
            ),
            pixels: create_pipeline(
                ctx,
                "pixels",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                None,
            ),
            clear_stencil: create_pipeline(
                ctx,
                "clear_stencil",
                layout,
                shader,
                format,
                wgpu::PrimitiveTopology::TriangleList,
                wgpu::Face::Back,
                None,
            ),
        };

        let triangles = create_pipeline(
            ctx,
            "triangles",
            layout,
            shader,
            format,
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::Face::Back,
            None,
        );

        // let convex_fill1 = create_pipeline(
        //     ctx,
        //     Some("convex_fill1"),
        //     shader,
        //     format,
        //     wgpu::PrimitiveTopology::TriangleStrip,
        //     wgpu::FrontFace::Ccw,
        //     None,
        //     None,
        // );
        // let stencil = ctx.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {

        // });
        // let pipeline = ctx.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //     label: Some("concave_fill1"),
        //     layout: None,
        //     vertex: wgpu::VertexState {
        //         module: shader,
        //         entry_point: "vs_main",
        //         buffers: &[],
        //     },
        //     fragment: Some(wgpu::FragmentState {
        //         module: shader,
        //         entry_point: "fs_main",
        //         //todo!
        //         targets: &[format.into()],
        //     }),
        //     primitive: wgpu::PrimitiveState {
        //         topology: wgpu::PrimitiveTopology::TriangleStrip,
        //         ..Default::default()
        //     },
        //     depth_stencil: Some(stroke_clear_stencil_state(format)),
        //     multisample: wgpu::MultisampleState::default(),
        // });

        todo!()
        // Self {

        // }
    }
}

// struct
pub struct WGPUPipelineCache {
    shader: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    // inner: std::rc::Rc<std::cell::RefCell<HashMap<PipelineCacheKey, WGPUPipelineState>>>,
    inner: std::cell::UnsafeCell<HashMap<PipelineCacheKey, WGPUPipelineStates>>,
    ctx: WGPUContext,
    // ph: &'a std::marker::PhantomData<()>,
}

impl WGPUPipelineCache {
    pub fn new(
        ctx: &WGPUContext,
        layout: wgpu::PipelineLayout,
        shader: wgpu::ShaderModule, // vert: &wgpu::
    ) -> Self {
        Self {
            shader,
            layout,
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
                &self.layout,
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
