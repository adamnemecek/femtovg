[[block]]
struct Vertex {
    pos: vec2<f32>;// [[attribute(0)]];
    // float2 tcoord [[attribute(1)]];
};

[[block]]
struct RasterizerData {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] fpos: vec2<f32>;
    [[location(1)]] ftcoord: vec2<f32>;
};

[[block]]
struct Uniforms {

    scissorMat: mat3x4<f32>;
    paintMat: mat3x4<f32>;
    innerCol: vec4<f32>;
    outerCol: vec4<f32>;
    scissorExt: vec4<f32>;
    scissorScale: vec4<f32>;
    extent: vec4<f32>;
    radius: f32;
    feather: f32;
    strokeMult: f32;
    strokeThr: f32;
    texType: f32;
    shaderType: f32;
    hasMask: f32;
    padding: array<f32, 19>;
};


fn scissorMask(u: Uniforms) -> f32 {

    return 0.0;
}

fn vertexShader(
    vert: Vertex,
    viewSize: vec2<f32>,
) {
    var ret: RasterizerData;
    ret.pos = vec4<f32>(
                    2.0 * vert.pos.x / viewSize.x - 1.0,
                    1.0 - 2.0 * vert.pos.y / viewSize.y,
                    0.0,
                    1.0
            );
    // return ret;
}

fn fragmentShaderAA() -> f32 {

    return 0.0;
}

// enum ShaderType {

// };