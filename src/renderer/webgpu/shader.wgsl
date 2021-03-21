// alias b="convert shader.wgsl shader.metal"
// [[block]]
// struct Vertex {
//     pos: vec2<f32>;// [[attribute(0)]];
//     // float2 tcoord [[attribute(1)]];
//     tcoord: vec2<f32>;
// };

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
    scissor_ext: vec2<f32>;
    scissor_scale: vec2<f32>;
    extent: vec2<f32>;
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

fn sdroundrect(u: Uniforms, pt: vec2<f32>) -> f32 {
    // float2 ext2 = uniforms.extent - float2(uniforms.radius);
    // float2 d = abs(pt) - ext2;
    // return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - uniforms.radius;
    return 0.0;
}

[[block]]
struct ViewSize {
    x: f32;
    y: f32;
};

// [[group(0), binding(0)]]
// var vert: Vertex;

[[group(0), binding(0)]]
var viewSize: ViewSize;

[[stage(vertex)]]
fn vertex_shader(
    [[location(0)]] vert: vec4<f32>,
    // [[location(1)]] v: Vertex,
    // vert: Vertex,
) -> RasterizerData {
    var ret: RasterizerData;
    const tcoord = vert.xy;
    const pos = vert.wz;
    ret.ftcoord = tcoord;
    ret.fpos = pos;
    ret.pos = vec4<f32>(
                    2.0 * pos.x / viewSize.x - 1.0,
                    1.0 - 2.0 * pos.y / viewSize.y,
                    0.0,
                    1.0
            );
    return ret;
}


// [[group(0), binding(1)]]
// var r_texture: texture_cube<f32>;
// [[group(0), binding(2)]]
// var r_sampler: sampler;

// todo: ordering


// [[group(0), binding(1)]]
// var i: RasterizerData;

[[group(0), binding(1)]]
var u: Uniforms;

[[group(0), binding(2)]]
var tex: texture_2d<f32>;
[[group(0), binding(3)]]
var samplr: sampler;
[[group(0), binding(4)]]
var alpha_tex: texture_2d<f32>;
[[group(0), binding(5)]]
var alpha_samplr: sampler;

[[stage(fragment)]]
fn fragment_shader_aa(
    in: RasterizerData,
    // u: Uniforms,

) -> [[location(0)]] vec4<f32> {

    var result: vec4<f32>;
    const scissor = scissor_mask(u, in.fpos);
    const stroke_alpha = stroke_mask(u, in.ftcoord);

    if (u.shader_type == 0.0) {
        // // MNVG_SHADER_FILLGRAD
        const pt = (u.paint_mat * vec3<f32>(in.fpos, 1.0)).xy;
        // // revisit d
        const d = clamp((sdroundrect(u, pt) + u.feather*0.5) / u.feather, 0.0, 1.0);
        // // float d = saturate((u.feather * 0.5 + sdroundrect(uniforms, pt))
        // //                    / u.feather);
        const color = mix(u.inner_col, u.outer_col, d);
        // // color *= scissor;
        // // color *= strokeAlpha;
        result = color;
    } elseif (u.shader_type == 1.0) {
        // MNVG_SHADER_IMG
        // this has to be fpos
        const pt = (u.paint_mat * vec3<f32>(in.fpos, 1.0)).xy / u.extent;

        var color: vec4<f32>;
        color = textureSample(tex, samplr, pt);

        if (u.tex_type == 1.0) {
            color = vec4<f32>(color.xyz * color.w, color.w);
        }
        elseif (u.tex_type == 2.0) {
            color = vec4<f32>(color.x, color.x, color.x, color.x);
        }
        result = color * u.inner_col;
    } else {
    //     // stencil
    //     // MNVG_SHADER_FILLIMG
        result = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }

    if (u.has_mask == 1.0) {
    //     // revisit ftcoord
        const ftcoord = vec2<f32>(in.ftcoord.x, 1.0 - in.ftcoord.y);
        const r = textureSample(alpha_tex, samplr, ftcoord).r;
        var mask: vec4<f32>;
        mask = vec4<f32>(r, r, r, r);
        // const mask = vec4<f32>(alpha_tex.sample(samplr, ftcoord).r);
        // const mask = textureSample(alpha_tex, samplr, vec2<u32>(0,0));

        mask = mask * scissor;
        result = result * mask;
    }
    elseif (u.shader_type != 2.0) {
        result = result * stroke_alpha * scissor;
    }

    if (stroke_alpha < u.stroke_thr) {
        discard;
    }

    return result;
}

// enum ShaderType {

// };