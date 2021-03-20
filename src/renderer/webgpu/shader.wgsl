[[block]]
struct Vertex {
    
};

[[block]]
struct RasterizerData {
    [[builtin(position)]] position: vec4<f32>;
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