pub trait WGPUDeviceExt {
    // fn create_buffer(&self)
    fn blit(&self);
}

impl WGPUDeviceExt for wgpu::CommandEncoder {
    fn blit(&self) {
        todo!()
    }
}

pub trait WGPUTextureExt {
    fn generate_mipmaps(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, texture: &wgpu::Texture);
}

impl WGPUTextureExt for wgpu::Texture {
    fn generate_mipmaps(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, texture: &wgpu::Texture) {
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
