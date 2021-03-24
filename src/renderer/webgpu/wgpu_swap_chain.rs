use super::Size;

use super::WGPUContext;

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
    pub fn new(ctx: &WGPUContext, size: Size) -> Self {
        let format = ctx.get_swap_chain_preferred_format();
        let desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format,
            width: size.w as _,
            height: size.h as _,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let inner = ctx.device().create_swap_chain(ctx.surface(), &desc);
        Self {
            size,
            inner,
            format,
        }
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
