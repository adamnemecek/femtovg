
// use fnv::FnvHashMap;
use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct RPSKey {
    // pub blend_func: Blend,
    pub pixel_format: wgpu::TextureFormat,
}

// struct 
pub struct WGPUPipelineCache {
    inner: HashMap<u32, u32>
}

impl WGPUPipelineCache {
    pub fn new() -> Self {
        todo!()
    }
}