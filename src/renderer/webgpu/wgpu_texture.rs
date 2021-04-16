use crate::{
    ErrorKind,
    ImageFlags,
    ImageInfo,
    ImageSource,
    PixelFormat,
    Size,
};

use super::{
    WGPUContext,
    WGPUDeviceExt,
    WGPUExtentExt,
    WGPUTextureExt,
};

// use std::sync::Once;

// static INSTANCE: Once<usize> = Once::new();

use rgb::ComponentBytes;

impl From<PixelFormat> for wgpu::TextureFormat {
    fn from(a: PixelFormat) -> Self {
        match a {
            PixelFormat::Rgba8 => Self::Rgba8Unorm,
            PixelFormat::Bgra8 => Self::Bgra8Unorm,
            PixelFormat::Rgb8 => unimplemented!("wgpu doesn't support the RGB8 pixel format"),
            PixelFormat::Gray8 => Self::R8Unorm,
        }
    }
}

pub struct WGPUTexture {
    //
    ctx: WGPUContext,
    info: ImageInfo,
    tex: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,

    // stencil: Option<wgpu::Texture>,
    // stencil_view: Option<wgpu::TextureView>,
    stencil: wgpu::Texture,
    stencil_view: wgpu::TextureView,
}

impl WGPUTexture {
    pub fn new_pseudo_texture(ctx: &WGPUContext) -> Result<Self, ErrorKind> {
        let info = ImageInfo::new(ImageFlags::empty(), 1, 1, PixelFormat::Gray8);
        Self::new(ctx, info, "pseudo texture")
    }

    pub fn new(ctx: &WGPUContext, info: ImageInfo, label: &str) -> Result<Self, ErrorKind> {
        assert!(info.format() != PixelFormat::Rgb8);
        let ctx = ctx.clone();

        let generate_mipmaps = info.flags().contains(ImageFlags::GENERATE_MIPMAPS);
        let nearest = info.flags().contains(ImageFlags::NEAREST);
        let repeatx = info.flags().contains(ImageFlags::REPEAT_X);
        let repeaty = info.flags().contains(ImageFlags::REPEAT_Y);

        let format = info.format().into();

        let size: wgpu::Extent3d = info.size().into();

        let mip_level_count = if generate_mipmaps {
            size.mip_mipmap_level_count()
        } else {
            1
        };

        // let sample_count = if generate_mipmaps { } else { 1 };
        // todo: what's the difference between texture and texture_view
        let tex = ctx.device().create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            //todo!
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::COPY_DST,
        });

        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        // .create_view(&Default::default());

        let filter = if nearest {
            wgpu::FilterMode::Nearest
        } else {
            wgpu::FilterMode::Linear
        };

        let mut sampler_desc = wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            mag_filter: filter,
            min_filter: filter,
            ..Default::default()
        };

        if generate_mipmaps {
            tex.generate_mipmaps(ctx.device());
            sampler_desc.mipmap_filter = filter;
        }

        sampler_desc.address_mode_u = if repeatx {
            wgpu::AddressMode::Repeat
        } else {
            wgpu::AddressMode::ClampToEdge
        };

        sampler_desc.address_mode_v = if repeaty {
            wgpu::AddressMode::Repeat
        } else {
            wgpu::AddressMode::ClampToEdge
        };

        let sampler = ctx.device().create_sampler(&sampler_desc);

        let stencil_label = format!("{:?} stencil", label);
        let stencil_desc = super::new_stencil_descriptor(info.size(), &stencil_label);

        // let (stencil, stencil_view) = if wants_stencil {
        let stencil = ctx.device().create_texture(&stencil_desc);
        let stencil_view = stencil.create_view(&Default::default());
        // (Some(stencil), Some(stencil_view))
        // }
        // else {
        // (None, None)
        // };

        Ok(Self {
            view,
            sampler,
            ctx,
            info,
            tex,
            stencil,
            stencil_view,
        })
    }

    pub fn stencil_view(&self) -> &wgpu::TextureView {
        &self.stencil_view
    }

    pub fn write_texture(&self, extent: wgpu::Extent3d, data: &[u8]) {
        // let layout = wgpu::TextureDataLayout { ..Default::default() };
        // self.context.queue().write_texture(&self.tex, data, layout, extent, )
        todo!()
    }

    pub fn resize(&mut self, size: Size) {
        // self.tex.destroy()
        todo!()
    }

    pub fn update(&mut self, src: ImageSource, x: usize, y: usize) -> Result<(), ErrorKind> {
        let (width, height) = src.dimensions();
        if x + width > self.info.width() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if y + height > self.info.height() {
            return Err(ErrorKind::ImageUpdateOutOfBounds);
        }

        if self.info.format() != src.format() {
            return Err(ErrorKind::ImageUpdateWithDifferentFormat);
        }

        let size = Size::new(width as _, height as _);
        let origin = wgpu::Origin3d {
            x: x as _,
            y: y as _,
            z: 0,
        };

        let copy_view = wgpu::ImageCopyTexture {
            mip_level: 0,
            origin,
            texture: self.tex(),
        };

        match src {
            ImageSource::Gray(data) => {
                let data_layout = wgpu::ImageDataLayout {
                    bytes_per_row: Some(std::num::NonZeroU32::new(width as u32)).unwrap(),
                    ..Default::default()
                };

                self.ctx
                    .queue()
                    .write_texture(copy_view, data.buf().as_bytes(), data_layout, size.into())
            }
            ImageSource::Rgba(data) => {
                let data_layout = wgpu::ImageDataLayout {
                    bytes_per_row: Some(std::num::NonZeroU32::new(4 * width as u32)).unwrap(),
                    ..Default::default()
                };

                self.ctx
                    .queue()
                    .write_texture(copy_view, data.buf().as_bytes(), data_layout, size.into())
            }
            ImageSource::Rgb(_) => {
                unimplemented!(
                    "wgpu doesn't support RGB pixel format. Image should have been converted in load_image_file"
                )
            }
        };
        let generate_mipmaps = self.info.flags().contains(ImageFlags::GENERATE_MIPMAPS);
        if generate_mipmaps {
            // self.tex.generate_mipmaps(&self.queue);
            self.tex().generate_mipmaps(self.ctx.device());
        }

        Ok(())
    }

    pub fn tex(&self) -> &wgpu::Texture {
        &self.tex
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn size(&self) -> Size {
        self.info.size()
    }

    // pub fn delete(&self) {
    //     // self.tex.destroy();
    //     // self.stencil.destroy();
    // }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.info.format().into()
    }
}

// impl Drop for WGPUTexture {
//     fn drop(&mut self) {
//         self.delete();
//     }
// }
