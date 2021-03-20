[[block]]
struct Vertex {
    pos: vec2<f32>;// [[attribute(0)]];
    // float2 tcoord [[attribute(1)]];
    tcoord: vec2<f32>;
};

// [[block]]
struct RasterizerData {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] fpos: vec2<f32>;
    [[location(1)]] ftcoord: vec2<f32>;
};

[[block]]
struct Uniforms {

    scissor_mat: mat3x4<f32>;
    paint_mat: mat3x4<f32>;
    inner_col: vec4<f32>;
    outer_col: vec4<f32>;
    scissor_ext: vec4<f32>;
    scissor_scale: vec4<f32>;
    extent: vec4<f32>;
    radius: f32;
    feather: f32;
    stroke_mult: f32;
    stroke_thr: f32;
    tex_type: f32;
    shader_type: f32;
    has_mask: f32;
    padding: array<f32, 19>;
};


fn scissor_mask(u: Uniforms, p: vec2<f32>) -> f32 {

    return 0.0;
}

fn stroke_mask(u: Uniforms, p: vec2<f32>) -> f32 {
    return 0.0;
}

[[block]]
struct ViewSize {
    x: f32;
    y: f32;
};

[[group(0), binding(0)]]
var vert: Vertex;

[[group(0), binding(1)]]
var viewSize: ViewSize;

[[stage(vertex)]]
fn vertex_shader(
    // vert: Vertex,
) -> RasterizerData {
    var ret: RasterizerData;
    ret.pos = vec4<f32>(
                    2.0 * vert.pos.x / viewSize.x - 1.0,
                    1.0 - 2.0 * vert.pos.y / viewSize.y,
                    0.0,
                    1.0
            );
    return ret;
}


// [[group(0), binding(1)]]
// var r_texture: texture_cube<f32>;
// [[group(0), binding(2)]]
// var r_sampler: sampler;

// var tex: texture,
// var samplr: sampler,
// var alpha_tex: texture,
// var alpha_samplr: sampler


[[group(0), binding(1)]]
var rd: RasterizerData;

[[group(0), binding(2)]]
var u: Uniforms;

[[stage(fragment)]]
fn fragment_shader_aa(
    // in: RasterizerData,
    // u: Uniforms,
    
) -> [[location(0)]] vec4<f32> {

    var result: vec4<f32>;

    const scissor = scissor_mask(u, rd.fpos);

    const stroke_alpha = stroke_mask(u, rd.ftcoord);
    if (stroke_alpha < u.stroke_thr) {
        discard;
    }



    // return textureSample(r_texture, r_sampler, in.uv);
    // return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    return result;
}

// enum ShaderType {

// };