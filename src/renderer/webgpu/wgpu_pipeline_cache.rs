use std::collections::HashMap;

use super::{
    Color,
    Rect,
    Vertex,
    WGPUBlend,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PipelineCacheKey {
    pub blend_func: WGPUBlend,
    pub texture_format: wgpu::TextureFormat,
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
                // format: wgpu::VertexFormat::Float4, // wgpu 0.7
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearRect {
    pub rect: Rect,
    pub color: Color,
    pub padding: [f32; 56],
}

impl ClearRect {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            color,
            padding: [0.0; 56],
        }
    }
}

impl ClearRect {
    // fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    //     use std::mem;
    //     wgpu::VertexBufferLayout {
    //         array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
    //         step_mode: wgpu::InputStepMode::Vertex,
    //         attributes: &[
    //             wgpu::VertexAttribute {
    //                 offset: 0,
    //                 shader_location: 0,
    //                 format: wgpu::VertexFormat::Float32x4,
    //             },
    //             // wgpu::VertexAttribute {
    //             //     offset: std::mem::size_of::<[f32; 4]>() as _,
    //             //     shader_location: 1,
    //             //     format: wgpu::VertexFormat::Float32x4,
    //             // },
    //             // wgpu::VertexAttribute {
    //             //     offset: 2 * std::mem::size_of::<[f32; 4]>() as _,
    //             //     shader_location: 1,
    //             //     format: wgpu::VertexFormat::Float32x4,
    //             // },
    //         ],
    //     }
    // }
}

// TODO: Remove this when updating to wgpu 0.7
impl From<WGPUBlend> for wgpu::BlendState {
    fn from(a: WGPUBlend) -> Self {
        Self {
            color: wgpu::BlendComponent {
                src_factor: a.src_rgb,
                dst_factor: a.dst_rgb,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: a.src_alpha,
                dst_factor: a.dst_alpha,
                operation: wgpu::BlendOperation::Add,
            },
        }
    }
}

fn create_pipeline<'a>(
    device: &wgpu::Device,
    label: impl Into<Option<&'a str>>,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    blend_func: WGPUBlend,
    topology: wgpu::PrimitiveTopology,
    strip_index_format: impl Into<Option<wgpu::IndexFormat>>,
    cull_mode: impl Into<Option<wgpu::Face>>,
    // cull_mode: impl Into<Option<wgpu::CullMode>>,  // wgpu 0.7
    depth_stencil: impl Into<Option<wgpu::DepthStencilState>>,
) -> wgpu::RenderPipeline {
    let label = label.into();
    let label = format!("{:?} {:?}", label, blend_func);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vertex_shader",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fragment_shader_aa",
            //todo!
            targets: &[wgpu::ColorTargetState {
                format,
                blend: Some(blend_func.into()),
                write_mask: wgpu::ColorWrite::all(),
            }],
        }),
        // front_face is ccw by default
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: strip_index_format.into(),

            cull_mode: cull_mode.into(),
            ..Default::default()
        },
        depth_stencil: depth_stencil.into(),
        multisample: wgpu::MultisampleState::default(),
    })

    /* wgpu 0.7
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vertex_shader",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fragment_shader_aa",
            //todo!
            targets: &[wgpu::ColorTargetState {
                format,
                color_blend: wgpu::BlendState {
                    src_factor: blend_func.src_rgb,
                    dst_factor: blend_func.dst_rgb,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendState {
                    src_factor: blend_func.src_alpha,
                    dst_factor: blend_func.dst_alpha,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::all(),
            }],
        }),
        // front_face is ccw by default
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: strip_index_format.into(),
            cull_mode: (cull_mode.into()).unwrap_or(wgpu::CullMode::default()),
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil: depth_stencil.into(),
        multisample: wgpu::MultisampleState::default(),
    })
    */
}

fn create_stencil_only_pipeline<'a>(
    device: &wgpu::Device,
    label: impl Into<Option<&'a str>>,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    blend_func: WGPUBlend,
    topology: wgpu::PrimitiveTopology,
    strip_index_format: impl Into<Option<wgpu::IndexFormat>>,
    cull_mode: impl Into<Option<wgpu::Face>>,
    // cull_mode: impl Into<Option<wgpu::CullMode>>,  // wgpu 0.7
    depth_stencil: impl Into<Option<wgpu::DepthStencilState>>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: label.into(),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vertex_shader",
            buffers: &[Vertex::desc()],
        },
        // fragment: None,
        // todo: in the original this is not set
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "passthrough",
            //todo!
            targets: &[wgpu::ColorTargetState {
                format,
                blend: Some(blend_func.into()),
                write_mask: wgpu::ColorWrite::empty(),
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: strip_index_format.into(),
            // front_face: wgpu::FrontFace::Ccw,
            cull_mode: cull_mode.into(),
            ..Default::default()
        },
        depth_stencil: depth_stencil.into(),
        multisample: wgpu::MultisampleState::default(),
    })

    /* wgpu 0.7
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: label.into(),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vertex_shader",
            buffers: &[Vertex::desc()],
        },
        // fragment: None,
        // todo: in the original this is not set
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "passthrough",
            //todo!
            targets: &[wgpu::ColorTargetState {
                format,
                color_blend: wgpu::BlendState {
                    src_factor: blend_func.src_rgb,
                    dst_factor: blend_func.dst_rgb,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendState {
                    src_factor: blend_func.src_alpha,
                    dst_factor: blend_func.dst_alpha,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::empty(),
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: strip_index_format.into(),
            cull_mode: (cull_mode.into()).unwrap_or(wgpu::CullMode::default()),
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil: depth_stencil.into(),
        multisample: wgpu::MultisampleState::default(),
    })
    */
}
fn create_clear_rect_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    stencil_format: wgpu::TextureFormat,
    layout: &wgpu::PipelineLayout,
) -> wgpu::RenderPipeline {
    // let b = wgpu::BlendComponent {
    //     src_factor: wgpu::BlendFactor::One,
    //     dst_factor: wgpu::BlendFactor::One,
    //     operation: wgpu::BlendOperation::Add,
    // };

    // let c = wgpu::BlendComponent {
    //     src_factor: wgpu::BlendFactor::One,
    //     dst_factor: wgpu::BlendFactor::OneMinusBlendColor,
    //     operation: wgpu::BlendOperation::Add,
    // };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("clear_rect"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vertex_clear_rect",
            buffers: &[
                // ClearRect::desc()
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fragment_clear_rect",
            //todo!
            targets: &[format.into()],
            // targets: &[wgpu::ColorTargetState {
            //     // format.into()
            //     format: format.into(),
            //     // blend: Some(wgpu::BlendState {
            //     //     color: wgpu::BlendComponent::REPLACE,
            //     //     alpha: wgpu::BlendComponent::REPLACE,
            //     // }),
            //     blend: Some(wgpu::BlendState {
            //         color: b,
            //         alpha: c,
            //     }),
            //     write_mask: wgpu::ColorWrite::ALL,
            // }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            // front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil: Some(default_stencil_state(stencil_format)),
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
    // println!("format {:?}", format);
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

                // fail_op: wgpu::StencilOperation::Keep,
                // depth_fail_op: wgpu::StencilOperation::Keep,
                // pass_op: wgpu::StencilOperation::Keep,
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

fn fill_anti_alias_stencil_state_evenodd(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Equal,
                ..Default::default()
                // fail_op: wgpu::StencilOperation::Keep,
                // depth_fail_op: wgpu::StencilOperation::Keep,
                // pass_op: wgpu::StencilOperation::Keep,
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
                // pass_op: wgpu::StencilOperation::Keep,
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

fn default_stencil_state(format: wgpu::TextureFormat) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState {
        format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Always,
        stencil: Default::default(),
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
    fills_nonzero: wgpu::RenderPipeline,
    fills_evenodd: wgpu::RenderPipeline,
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

    pub fn fills_nonzero(&self) -> &wgpu::RenderPipeline {
        &self.fills_nonzero
    }

    pub fn fills_evenodd(&self) -> &wgpu::RenderPipeline {
        &self.fills_evenodd
    }
}

pub struct StencilStroke {
    stroke_base: wgpu::RenderPipeline,
    aa_pixels: wgpu::RenderPipeline,
    clear_stencil: wgpu::RenderPipeline,
}

impl StencilStroke {
    pub fn stroke_base(&self) -> &wgpu::RenderPipeline {
        &self.stroke_base
    }

    pub fn aa_pixels(&self) -> &wgpu::RenderPipeline {
        &self.aa_pixels
    }

    pub fn clear_stencil(&self) -> &wgpu::RenderPipeline {
        &self.clear_stencil
    }
}

pub struct WGPUPipelineStates {
    blend_func: WGPUBlend,
    format: wgpu::TextureFormat,

    convex_fill: ConvexFill,
    concave_fill: ConcaveFill,
    stroke: wgpu::RenderPipeline,
    stencil_stroke: StencilStroke,
    triangles: wgpu::RenderPipeline,
    clear_rect: wgpu::RenderPipeline,
}

impl WGPUPipelineStates {
    pub fn matches(&self, blend_func: WGPUBlend, format: wgpu::TextureFormat) -> bool {
        self.blend_func == blend_func && self.format == format
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

    pub fn concave_fill(&self) -> &ConcaveFill {
        &self.concave_fill
    }

    pub fn stroke(&self) -> &wgpu::RenderPipeline {
        &self.stroke
    }

    pub fn stencil_stroke(&self) -> &StencilStroke {
        &self.stencil_stroke
    }

    pub fn triangles(&self) -> &wgpu::RenderPipeline {
        &self.triangles
    }

    pub fn clear_rect(&self) -> &wgpu::RenderPipeline {
        &self.clear_rect
    }

    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        clear_rect_layout: &wgpu::PipelineLayout,
        blend_func: WGPUBlend,
        format: wgpu::TextureFormat,
        shader: &wgpu::ShaderModule,
        // vertex_desc: &wgpu::VertexBufferLayout,
    ) -> Self {
        let stencil_format = wgpu::TextureFormat::Depth24PlusStencil8;
        // let default_stencil_state = default_stencil_state(stencil_format);
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
            // todo: i'm not sure if this should be a trianglelist or a triangle strip
            // in metal, we are using indexed rendering
            fill_buffer: create_pipeline(
                device,
                "convex_fill/fill_buffer",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleList,
                // None,
                // wgpu::IndexFormat::Uint32,
                None,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                default_stencil_state(stencil_format),
            ),
            stroke_buffer: create_pipeline(
                device,
                "convex_fill/stroke_buffer",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                default_stencil_state(stencil_format),
            ),
        };

        let concave_fill = ConcaveFill {
            // stencil only pipeline state
            fill_verts: create_stencil_only_pipeline(
                device,
                "concave_fill/fill_verts",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleList,
                None,
                None,
                fill_shape_stencil_state(stencil_format),
            ),
            fringes_nonzero: create_pipeline(
                device,
                "concave_fill/fringes_nonzero",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                fill_anti_alias_stencil_state_nonzero(stencil_format),
            ),
            fringes_evenodd: create_pipeline(
                device,
                "concave_fill/fringes_evenodd",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                fill_anti_alias_stencil_state_evenodd(stencil_format),
            ),
            fills_nonzero: create_pipeline(
                device,
                "concave_fill/fills_nonzero",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                fill_stencil_state_nonzero(stencil_format),
            ),
            fills_evenodd: create_pipeline(
                device,
                "concave_fill/fills_evenodd",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                fill_stencil_state_evenodd(stencil_format),
            ),
        };

        let stroke = create_pipeline(
            device,
            "stroke",
            layout,
            shader,
            format,
            blend_func,
            wgpu::PrimitiveTopology::TriangleStrip,
            wgpu::IndexFormat::Uint32,
            wgpu::Face::Back,
            // wgpu::CullMode::Back,  // wgpu 0.7
            default_stencil_state(stencil_format),
        );

        let stencil_stroke = StencilStroke {
            stroke_base: create_pipeline(
                device,
                "stroke_base",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                stroke_shape_stencil_state(stencil_format),
            ),
            aa_pixels: create_pipeline(
                device,
                "aa_pixels",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                stroke_anti_alias_stencil_state(stencil_format),
            ),
            clear_stencil: create_stencil_only_pipeline(
                device,
                "clear_stencil",
                layout,
                shader,
                format,
                blend_func,
                wgpu::PrimitiveTopology::TriangleStrip,
                wgpu::IndexFormat::Uint32,
                wgpu::Face::Back,
                // wgpu::CullMode::Back,  // wgpu 0.7
                stroke_clear_stencil_state(stencil_format),
            ),
        };

        let triangles = create_pipeline(
            device,
            "triangles",
            layout,
            shader,
            format,
            blend_func,
            wgpu::PrimitiveTopology::TriangleList,
            None,
            wgpu::Face::Back,
            // wgpu::CullMode::Back,  // wgpu 0.7
            default_stencil_state(stencil_format),
        );

        let clear_rect = create_clear_rect_pipeline(device, shader, format, stencil_format, clear_rect_layout);

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
        // let stencil = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {

        // });
        // let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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

        // todo!()
        Self {
            blend_func,
            format,
            convex_fill,
            concave_fill,
            stroke,
            stencil_stroke,
            triangles,
            clear_rect,
        }
    }
}

// struct
pub struct WGPUPipelineCache {
    shader: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    clear_rect_layout: wgpu::PipelineLayout,
    // inner: std::rc::Rc<std::cell::RefCell<HashMap<PipelineCacheKey, WGPUPipelineState>>>,
    inner: std::cell::UnsafeCell<HashMap<PipelineCacheKey, WGPUPipelineStates>>,
    // ph: &'a std::marker::PhantomData<()>,
}

impl WGPUPipelineCache {
    pub fn new(
        layout: wgpu::PipelineLayout,
        clear_rect_layout: wgpu::PipelineLayout,
        shader: wgpu::ShaderModule, // vert: &wgpu::
    ) -> Self {
        Self {
            shader,
            layout,
            clear_rect_layout,
            inner: Default::default(),
        }
    }

    fn inner(&self) -> &mut HashMap<PipelineCacheKey, WGPUPipelineStates> {
        unsafe { self.inner.get().as_mut().unwrap() }
    }

    pub fn get<'a>(&'a self, device: &wgpu::Device, blend_func: WGPUBlend, texture_format: wgpu::TextureFormat) -> &'a WGPUPipelineStates {
        let key = PipelineCacheKey {
            blend_func,
            texture_format,
        };
        let r = self.inner();

        if !r.contains_key(&key) {
            let ps = WGPUPipelineStates::new(
                device,
                &self.layout,
                &self.clear_rect_layout,
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

    pub fn clear(&mut self) {
        self.inner().clear()
    }
}
