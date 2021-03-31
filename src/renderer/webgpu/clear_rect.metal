#include <metal_stdlib>
#include <simd/simd.h>

typedef float type;
struct Rect {
    type x;
    type y;
    type w;
    type h;
};
typedef metal::float4 type1;
typedef int type2;
typedef metal::float2 type3;
typedef type type4[const_56i];
struct ClearRectIn {
    type1 rect;
    type1 color;
    type4 padding;
};
struct ClearRectOut {
    type1 pos;
    type1 color1;
};
typedef uint type5;
constexpr constant float const_0f = 0.0;
constexpr constant int const_56i = 56;
constexpr constant float const_1f = 1.0;
type3 rect_vert_cw(
    type1 rect1,
    type2 vid
) {
    type3 pos1;
    float _expr7 = (rect1.x + rect1.z);
    float _expr8 = (rect1.y + rect1.w);
    switch(vid) {
        case 0: {
            pos1 = metal::float2(_expr7, _expr8);
        }
        case 1: {
            pos1 = metal::float2(rect1.x, _expr8);
        }
        case 2: {
            pos1 = metal::float2(_expr7, rect1.y);
        }
        case 3: {
            pos1 = metal::float2(rect1.x, rect1.y);
        }
        default: {
            pos1 = metal::float2(const_0f, const_0f);
        }
    }
    return pos1;
}

struct vertex_clear_rectInput {
};
struct vertex_clear_rectOutput {
    type1 pos [[position]];
    type1 color1 [[user(loc0)]];
};
vertex vertex_clear_rectOutput vertex_clear_rect(
  type5 vid1 [[vertex_id]]
, constant ClearRectIn& u [[user(fake0)]]
) {
    ClearRectOut out;
    type3 _expr5 = rect_vert_cw(u.rect, static_cast<int>(vid1));
    out.pos = metal::float4(_expr5, const_0f, const_1f);
    out.color1 = u.color;
    const auto _tmp = out;
    return vertex_clear_rectOutput { _tmp.pos, _tmp.color1 };
}

struct fragment_clear_rectInput {
    type1 color1 [[user(loc0)]];
};
struct fragment_clear_rectOutput {
    type1 member1 [[color(0)]];
};
fragment fragment_clear_rectOutput fragment_clear_rect(
  fragment_clear_rectInput varyings1 [[stage_in]]
, type1 pos [[position]]
) {
    const ClearRectOut in = { pos, varyings1.color1 };
    return fragment_clear_rectOutput { in.color1 };
}
