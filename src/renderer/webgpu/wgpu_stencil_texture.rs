use super::WGPUContext;
use crate::geometry::Size;
pub struct WGPUStencilTexture {
    //
    ctx: WGPUContext,
    size: Size,
}

impl WGPUStencilTexture {
    pub fn new(ctx: &WGPUContext, size: Size) -> Self {
        todo!()
        // Self {

        // }
    }

    pub fn resize(&mut self, size: Size) {
        todo!()
    }

    pub fn view(&self) -> &wgpu::TextureView {
        todo!()
    }
}
