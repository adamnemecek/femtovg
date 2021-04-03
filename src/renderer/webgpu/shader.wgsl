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


// size_of(uniforms) == 256
[[block]]
struct Uniforms {
    scissor_mat: mat3x4<f32>;                       // 3 * 4 * 4
    paint_mat: mat3x4<f32>;                         // 3 * 4 * 4
    inner_col: vec4<f32>;                           // 4 * 4
    outer_col: vec4<f32>;                           // 4 * 4
    scissor_ext: vec2<f32>;                         // 2 * 4
    scissor_scale: vec2<f32>;                       // 2 * 4
    extent: vec2<f32>;                              // 2 * 4
    radius: f32;                                    // 4
    feather: f32;                                   // 4
    stroke_mult: f32;                               // 4
    stroke_thr: f32;                                // 4
    tex_type: f32;                                  // 4
    shader_type: f32;                               // 4
    has_mask: f32;                                  // 4
    padding: [[stride(4)]] array<f32, 19>;          // 19 * 4
};

fn scissor_mask(u: Uniforms, p: vec2<f32>) -> f32 {
    var sc: vec2<f32>;
    sc = (abs((u.scissor_mat * vec3<f32>(p, 1.0)).xy)
                 - u.scissor_ext);

    sc = vec2<f32>(0.5, 0.5) - sc * u.scissor_scale;
    return clamp(sc.x, 0.0, 1.0) * clamp(sc.y, 0.0, 1.0);
}

fn stroke_mask(u: Uniforms, ftcoord: vec2<f32>) -> f32 {
    return min(1.0, (1.0 - abs(ftcoord.x * 2.0 - 1.0)) * u.stroke_mult)
          * min(1.0, ftcoord.y);
    // return 0.0;
}

fn sdroundrect(u: Uniforms, pt: vec2<f32>) -> f32 {
    // float2 ext2 = uniforms.extent - float2(uniforms.radius);
    // float2 d = abs(pt) - ext2;
    // return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - uniforms.radius;
    // return 0.0;
    const ext2 = u.extent - vec2<f32>(u.radius, u.radius);
    const d = abs(pt) - ext2;
    // return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - u.radius;
    return min(max(d.x, d.y), 0.0) + length(vec2<f32>(max(d.x, 0.0), max(d.y, 0.0))) - u.radius;
}

[[block]]
struct ViewSize {
    x: u32;
    y: u32;
};

var<push_constant> viewSize: ViewSize;

// [[group(0), binding(0)]]
// var viewSize: ViewSize;

[[stage(vertex)]]
fn vertex_shader(
    [[location(0)]] vert: vec4<f32>,
) -> RasterizerData {
    const pos = vert.xy;
    const tcoord = vert.zw;

    var ret: RasterizerData;
    ret.ftcoord = tcoord;
    ret.fpos = pos;
    ret.pos = vec4<f32>(
                    2.0 * pos.x / f32(viewSize.x) - 1.0,
                    1.0 - 2.0 * pos.y / f32(viewSize.y),
                    0.0,
                    1.0
            );
    return ret;
}

// [[group(0), binding(1)]]
// var u: Uniforms;
// var<push_constant> u: Uniforms;
[[group(0), binding(0)]]
var<uniform> u: Uniforms;

[[group(0), binding(1)]]
var tex: texture_2d<f32>;
[[group(0), binding(2)]]
var samplr: sampler;
[[group(0), binding(3)]]
var alpha_tex: texture_2d<f32>;
[[group(0), binding(4)]]
var alpha_samplr: sampler;

[[stage(fragment)]]
fn fragment_shader_aa(
    in: RasterizerData,
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
        // const color = mix(u.inner_col, u.outer_col, d);
         const color = vec4<f32>(
           mix(u.inner_col.r, u.outer_col.r, d),
           mix(u.inner_col.g, u.outer_col.g, d),
           mix(u.inner_col.b, u.outer_col.b, d),
           mix(u.inner_col.a, u.outer_col.a, d)
         );
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
        const r = textureSample(alpha_tex, alpha_samplr, ftcoord).x;
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



[[stage(fragment)]]
fn passthrough(
    in: RasterizerData,
) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}

struct Rect {
    x: f32;
    y: f32;
    w: f32;
    h: f32;
};



fn rect_vert_cw(
    rect: vec4<f32>,
    vid: i32
) -> vec2<f32> {
    var pos: vec2<f32>;
    const x = rect.x;
    const y = rect.y;
    const w = rect.z;
    const h = rect.w;

    const left: f32 = x;
    const right: f32 = x + w;
    const bottom: f32 = y;
    const top: f32 = y + h;

    // switch (vid) {
    //     case 0: {
    //         pos = vec2<f32>(right, top);
    //     }
    //     case 1: {
    //         pos = vec2<f32>(left, top);
    //     }
    //     case 2: {
    //         pos = vec2<f32>(right, bottom);
    //     }
    //     case 3: {
    //         pos = vec2<f32>(left, bottom);
    //     }
    //     default: {
    //         pos = vec2<f32>(0.0, 0.0);
    //     }
    // };
    if (vid == 0) {
        pos = vec2<f32>(right, top);
    } elseif (vid == 1) {
        pos = vec2<f32>(left, top);
    } elseif (vid == 2) {
        pos = vec2<f32>(right, bottom);
    } elseif (vid == 3) {
        pos = vec2<f32>(left, bottom);
    } else {
        pos = vec2<f32>(0.0, 0.0);
    }
    // de
    return pos;
}

[[block]]
struct ClearRectIn {
    rect: vec4<f32>;
    color: vec4<f32>;
    padding: [[stride(4)]] array<f32, 56>;
};

// [[block]]
struct ClearRectOut {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};


[[group(0), binding(0)]]
var<uniform> u: ClearRectIn;

[[stage(vertex)]]
fn vertex_clear_rect(
    [[builtin(vertex_index)]] vid: u32,
) -> ClearRectOut {
    const pos = rect_vert_cw(u.rect, i32(vid));

    var out: ClearRectOut;
    out.pos = vec4<f32>(pos, 0.0, 1.0);
    out.color = u.color;

    return out;
}

[[stage(fragment)]]
fn fragment_clear_rect(
    in: ClearRectOut
) -> [[location(0)]] vec4<f32> {
    return in.color;
}

