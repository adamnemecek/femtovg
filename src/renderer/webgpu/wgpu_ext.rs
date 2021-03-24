pub trait WGPUDeviceExt {
    // fn create_buffer(&self)
    fn blit(&self);
}

impl WGPUDeviceExt for wgpu::CommandEncoder {
    fn blit(&self) {
        todo!()
    }
}

pub trait RenderPassExt {
    // #[must_use]
    fn set_vertex_value<T: Copy>(&mut self, offset: u32, value: &T) -> u32;

    // #[must_use]
    fn set_fragment_value<T: Copy>(&mut self, offset: u32, value: &T) -> u32;
}

// pub trait ByteSliceExt {
//     fn from_value<T>(v: &T) -> Self;
// }

// impl ByteSliceExt for &[u8] {
//     fn from_value<T>(v: &T) -> Self {
//         unsafe {
//             std::slice::from_raw_parts(value as *const T as *const u8, size)
//        }
//     }
// }

impl<'a> RenderPassExt for wgpu::RenderPass<'a> {
    fn set_vertex_value<T: Copy>(&mut self, offset: u32, value: &T) -> u32 {
        let size = std::mem::size_of::<T>();
        debug_assert!(size % 4 == 0);
        let slice = unsafe { std::slice::from_raw_parts(value as *const T as *const u8, size) };
        self.set_push_constants(wgpu::ShaderStage::VERTEX, offset, slice);
        size as _
    }

    fn set_fragment_value<T: Copy>(&mut self, offset: u32, value: &T) -> u32 {
        let size = std::mem::size_of::<T>();
        debug_assert!(size % 4 == 0);
        let slice = unsafe { std::slice::from_raw_parts(value as *const T as *const u8, size) };
        self.set_push_constants(wgpu::ShaderStage::FRAGMENT, offset, slice);
        size as _
    }
}

pub trait WGPUExtentExt {
    fn mip_mipmap_level_count(&self) -> u32;
}

impl WGPUExtentExt for wgpu::Extent3d {
    fn mip_mipmap_level_count(&self) -> u32 {
        let Self {
            width,
            height,
            depth_or_array_layers: depth,
        } = self;
        (*(width.max(height).max(depth)) as u64 as f64).log2().ceil() as u32
    }
}

pub trait WGPUTextureExt {
    fn generate_mipmaps(&self, device: &wgpu::Device); //, encoder: &mut wgpu::CommandEncoder);
}

impl WGPUTextureExt for wgpu::Texture {
    fn generate_mipmaps(&self, device: &wgpu::Device) {
        //, encoder: &mut wgpu::CommandEncoder) {
        todo!()
        // assert_eq!(texture.descriptor.array_layer_count, 1);
        // assert!(texture.descriptor.mip_level_count > 1);

        // let mut prev_level = texture.get_mip_level_view(0);
        // let mut width = texture.descriptor.size.width;
        // let mut height = texture.descriptor.size.height;

        // let mut bind_groups = vec![];
        // let mut pass = encoder.begin_compute_pass();
        // pass.set_pipeline(&self.pipeline);

        // for mip_level in 1..texture.descriptor.mip_level_count {
        //     let current_level = texture.get_mip_level_view(mip_level);

        //     let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //         layout: &self.bind_group_layout,
        //         bindings: &[
        //             wgpu::Binding {
        //                 binding: 0,
        //                 resource: wgpu::BindingResource::TextureView(&prev_level),
        //             },
        //             wgpu::Binding {
        //                 binding: 1,
        //                 resource: wgpu::BindingResource::TextureView(&current_level),
        //             },
        //         ],
        //         label: None,
        //     });
        //     bind_groups.push(bind_group);
        //     prev_level = current_level;
        // }

        // for bind_group in &bind_groups {
        //     width = (width / 2).max(1);
        //     height = (height / 2).max(1);
        //     let local_size: u32 = 8;
        //     {
        //         pass.set_bind_group(0, bind_group, &[]);
        //         pass.dispatch(
        //             (width + local_size - 1) / local_size,
        //             (height + local_size - 1) / local_size,
        //             1,
        //         );
        //     }
        // }
    }
}
