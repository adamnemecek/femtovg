#include <metal_stdlib>
#include <simd/simd.h>

typedef metal::float4 type;
typedef metal::float2 type1;
struct RasterizerData {
    type pos;
    type1 fpos;
    type1 ftcoord;
};
typedef metal::float3x4 type2;
typedef float type3;
typedef type3 type4[const_19i];
struct Uniforms {
    type2 scissor_mat;
    type2 paint_mat;
    type inner_col;
    type outer_col;
    type1 scissor_ext;
    type1 scissor_scale;
    type1 extent;
    type3 radius;
    type3 feather;
    type3 stroke_mult;
    type3 stroke_thr;
    type3 tex_type;
    type3 shader_type;
    type3 has_mask;
    type4 padding;
};
typedef metal::float3 type5;
typedef uint type6;
struct ViewSize {
    type6 x;
    type6 y;
};
typedef metal::texture2d<float, metal::access::sample> type7;
typedef metal::sampler type8;
struct Rect {
    type3 x1;
    type3 y1;
    type3 w;
    type3 h;
};
typedef int type9;
typedef type3 type10[const_56i];
struct ClearRectIn {
    type rect;
    type color;
    type10 padding1;
};
struct ClearRectOut {
    type pos1;
    type color1;
};
constexpr constant int const_19i = 19;
constexpr constant float const_1f = 1.0;
constexpr constant float const_0_50f = 0.5;
constexpr constant float const_0f = 0.0;
constexpr constant float const_2f = 2.0;
constexpr constant int const_0i = 0;
constexpr constant int const_1i = 1;
constexpr constant int const_2i = 2;
constexpr constant int const_3i = 3;
constexpr constant int const_56i = 56;
type3 scissor_mask(
    Uniforms u2,
    type1 p
) {
    type1 sc;
    metal::float4 _expr6 = (u2.scissor_mat * metal::float3(p, const_1f));
    sc = (metal::abs(metal::float2(_expr6.x, _expr6.y)) - u2.scissor_ext);
    sc = (metal::float2(const_0_50f, const_0_50f) - (sc * u2.scissor_scale));
    return (metal::clamp(sc.x, const_0f, const_1f) * metal::clamp(sc.y, const_0f, const_1f));
}

type3 stroke_mask(
    Uniforms u3,
    type1 ftcoord1
) {
    return (metal::min(const_1f, ((const_1f - metal::abs(((ftcoord1.x * const_2f) - const_1f))) * u3.stroke_mult)) * metal::min(const_1f, ftcoord1.y));
}

type3 sdroundrect(
    Uniforms u4,
    type1 pt
) {
    type1 _expr8 = (metal::abs(pt) - (u4.extent - metal::float2(u4.radius, u4.radius)));
    return ((metal::min(metal::max(_expr8.x, _expr8.y), const_0f) + metal::length(metal::float2(metal::max(_expr8.x, const_0f), metal::max(_expr8.y, const_0f)))) - u4.radius);
}

type1 rect_vert_cw(
    type rect1,
    type9 vid
) {
    type1 pos2;
    float _expr13 = (rect1.x + rect1.z);
    float _expr14 = (rect1.y + rect1.w);
    if ((vid == const_0i)) {
        pos2 = metal::float2(_expr13, _expr14);
    } else {
        if ((vid == const_1i)) {
            pos2 = metal::float2(rect1.x, _expr14);
        } else {
            if ((vid == const_2i)) {
                pos2 = metal::float2(_expr13, rect1.y);
            } else {
                if ((vid == const_3i)) {
                    pos2 = metal::float2(rect1.x, rect1.y);
                } else {
                    pos2 = metal::float2(const_0f, const_0f);
                }
            }
        }
    }
    return pos2;
}

struct vertex_shaderInput {
    type vert [[attribute(0)]];
};
struct vertex_shaderOutput {
    type pos [[position]];
    type1 fpos [[user(loc0)]];
    type1 ftcoord [[user(loc1)]];
};
vertex vertex_shaderOutput vertex_shader(
  vertex_shaderInput varyings [[stage_in]]
, ViewSize viewSize
) {
    const auto vert = varyings.vert;
    RasterizerData ret;
    type1 _expr4 = metal::float2(vert.x, vert.y);
    ret.ftcoord = metal::float2(vert.z, vert.w);
    ret.fpos = _expr4;
    ret.pos = metal::float4((((const_2f * _expr4.x) / static_cast<float>(viewSize.x)) - const_1f), (const_1f - ((const_2f * _expr4.y) / static_cast<float>(viewSize.y))), const_0f, const_1f);
    const auto _tmp = ret;
    return vertex_shaderOutput { _tmp.pos, _tmp.fpos, _tmp.ftcoord };
}

struct fragment_shader_aaInput {
    type1 fpos [[user(loc0)]];
    type1 ftcoord [[user(loc1)]];
};
struct fragment_shader_aaOutput {
    type member1 [[color(0)]];
};
fragment fragment_shader_aaOutput fragment_shader_aa(
  fragment_shader_aaInput varyings1 [[stage_in]]
, type pos [[position]]
, constant Uniforms& u [[user(fake0)]]
, type7 tex [[user(fake0)]]
, type8 samplr [[user(fake0)]]
, type7 alpha_tex [[user(fake0)]]
, type8 alpha_samplr [[user(fake0)]]
) {
    const RasterizerData in = { pos, varyings1.fpos, varyings1.ftcoord };
    type result;
    type color2;
    type mask;
    type3 _expr10 = scissor_mask(u, in.fpos);
    type3 _expr13 = stroke_mask(u, in.ftcoord);
    if ((u.shader_type == const_0f)) {
        metal::float4 _expr23 = (u.paint_mat * metal::float3(in.fpos, const_1f));
        type3 _expr28 = sdroundrect(u, metal::float2(_expr23.x, _expr23.y));
        type3 _expr39 = metal::clamp(((_expr28 + (u.feather * const_0_50f)) / u.feather), const_0f, const_1f);
        result = metal::float4(metal::mix(u.inner_col.x, u.outer_col.x, _expr39), metal::mix(u.inner_col.y, u.outer_col.y, _expr39), metal::mix(u.inner_col.z, u.outer_col.z, _expr39), metal::mix(u.inner_col.w, u.outer_col.w, _expr39));
    } else {
        if ((u.shader_type == const_1f)) {
            metal::float4 _expr78 = (u.paint_mat * metal::float3(in.fpos, const_1f));
            metal::float4 _expr86 = tex.sample(samplr, (metal::float2(_expr78.x, _expr78.y) / u.extent));
            color2 = _expr86;
            if ((u.tex_type == const_1f)) {
                type _expr91 = color2;
                color2 = metal::float4((metal::float3(_expr91.x, _expr91.y, _expr91.z) * color2.w), color2.w);
            } else {
                if ((u.tex_type == const_2f)) {
                    color2 = metal::float4(color2.x, color2.x, color2.x, color2.x);
                }
            }
            result = (color2 * u.inner_col);
        } else {
            result = metal::float4(const_1f, const_1f, const_1f, const_1f);
        }
    }
    if ((u.has_mask == const_1f)) {
        metal::float4 _expr135 = alpha_tex.sample(alpha_samplr, metal::float2(in.ftcoord.x, (const_1f - in.ftcoord.y)));
        mask = metal::float4(_expr135.x, _expr135.x, _expr135.x, _expr135.x);
        mask = (mask * _expr10);
        result = (result * mask);
    } else {
        if ((u.shader_type != const_2f)) {
            result = ((result * _expr13) * _expr10);
        }
    }
    if ((_expr13 < u.stroke_thr)) {
        metal::discard_fragment();
    }
    return fragment_shader_aaOutput { result };
}

struct passthroughInput {
    type1 fpos [[user(loc0)]];
    type1 ftcoord [[user(loc1)]];
};
struct passthroughOutput {
    type member2 [[color(0)]];
};
fragment passthroughOutput passthrough(
  passthroughInput varyings2 [[stage_in]]
, type pos [[position]]
) {
    const RasterizerData in1 = { pos, varyings2.fpos, varyings2.ftcoord };
    return passthroughOutput { metal::float4(const_0f, const_0f, const_0f, const_0f) };
}

struct vertex_clear_rectInput {
};
struct vertex_clear_rectOutput {
    type pos1 [[position]];
    type color1 [[user(loc0)]];
};
vertex vertex_clear_rectOutput vertex_clear_rect(
  type6 vid1 [[vertex_id]]
, constant ClearRectIn& u1 [[user(fake0)]]
) {
    ClearRectOut out;
    type1 _expr10 = rect_vert_cw(u1.rect, static_cast<int>(vid1));
    out.pos1 = metal::float4(_expr10, const_0f, const_1f);
    out.color1 = u1.color;
    const auto _tmp = out;
    return vertex_clear_rectOutput { _tmp.pos1, _tmp.color1 };
}

struct fragment_clear_rectInput {
    type color1 [[user(loc0)]];
};
struct fragment_clear_rectOutput {
    type member4 [[color(0)]];
};
fragment fragment_clear_rectOutput fragment_clear_rect(
  fragment_clear_rectInput varyings4 [[stage_in]]
, type pos1 [[position]]
) {
    const ClearRectOut in2 = { pos1, varyings4.color1 };
    return fragment_clear_rectOutput { in2.color1 };
}
