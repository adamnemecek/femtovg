use super::Size;

fn sc_desc(format: wgpu::TextureFormat, size: Size) -> wgpu::SwapChainDescriptor {
    wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format,
        width: size.w as _,
        height: size.h as _,
        present_mode: wgpu::PresentMode::Mailbox,
    }
}

pub struct WGPUSwapChain {
    size: Size,
    inner: wgpu::SwapChain,
    format: wgpu::TextureFormat,
}

impl WGPUSwapChain {
    pub fn new(format: wgpu::TextureFormat, size: Size) -> Self {
        todo!()
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    pub fn get_current_frame(&self) -> Result<wgpu::SwapChainFrame, wgpu::SwapChainError> {
        self.inner.get_current_frame()
    }

    pub fn resize(&self, size: Size) {
        let desc = sc_desc(self.format, self.size);
        todo!()
    }
}
