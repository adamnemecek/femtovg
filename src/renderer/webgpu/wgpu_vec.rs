use wgpu::util::DeviceExt;

use super::{
    MemAlign,
    WGPUContext,
    WGPUInstance,
    WGPUVar,
};

pub struct WGPUVecViewMut<'a, T: Copy> {
    // len:
    inner: wgpu::BufferViewMut<'a>,
    ph: std::marker::PhantomData<T>,
}

// impl<'a, T: Copy> WGPUVecViewMut<'a, T> {
//     pub fn new(v: WGUPVec<T>) -> Self {
//         todo!()
//     }
// }

// pub struct WGPUVecIterator<'a, T: Copy> {
//     inner: &'a WGPUVec<T>,
//     idx: usize,
// }

// impl<'a, T: Copy> WGPUVecIterator<'a, T> {
//     fn new(inner: &'a WGPUVec<T>) -> Self {
//         Self { inner, idx: 0 }
//     }
// }

// impl<'a, T: Copy> Iterator for WGPUVecIterator<'a, T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx >= self.inner.len() {
//             None
//         } else {
//             let res = self.inner[self.idx];
//             self.idx += 1;
//             Some(res)
//         }
//     }
// }

pub fn as_u8_slice<T>(v: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of::<T>() * v.len()) }
}

fn create_buffer<T: Copy>(
    ctx: &WGPUContext,
    label: &str,
    mem_align: MemAlign<T>,
    usage: wgpu::BufferUsage,
) -> wgpu::Buffer {
    ctx.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
         /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
        // pub label: L,
        /// Size of a buffer.
        // pub size: BufferAddress,
        size: mem_align.byte_size as _,
        /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
        /// will panic.
        // pub usage: BufferUsage,
        usage,//: wgpu::BufferUsage::COPY_DST,
        /// Allows a buffer to be mapped immediately after they are made. It does not have to be [`BufferUsage::MAP_READ`] or
        /// [`BufferUsage::MAP_WRITE`], all buffers are allowed to be mapped at creation.
        // pub mapped_at_creation: bool,
        mapped_at_creation: false,
    })
}

pub struct WGPUVec<T: Copy> {
    ctx: WGPUContext,
    inner: wgpu::Buffer,
    // len: usize,
    mem_align: MemAlign<T>,
    usage: wgpu::BufferUsage,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResizeResult {
    None,
    Resized,
}

impl ResizeResult {
    pub fn resized(&self) -> bool {
        self == &Self::Resized
    }
}

impl<T: Copy> WGPUVec<T> {
    pub fn new_vertex(ctx: &WGPUContext, capacity: usize) -> Self {
        let mem_align = MemAlign::new(capacity);

        let usage = wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST;
        let inner = create_buffer(ctx, "vertex buffer", mem_align, usage);

        Self {
            usage,
            ctx: ctx.clone(),
            inner,
            // len: 0,
            mem_align,
        }
    }

    pub fn new_uniform(ctx: &WGPUContext, capacity: usize) -> Self {
        let mem_align = MemAlign::new(capacity);

        let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let inner = create_buffer(ctx, "uniform buffer", mem_align, usage);

        Self {
            usage,
            ctx: ctx.clone(),
            inner,
            // len: 0,
            mem_align,
        }
    }

    // pub fn map_async(&self) -> impl std::future::Future<Output = Result<(), wgpu::BufferAsyncError>> + Send {
    //     self.slice().map_async(wgpu::MapMode::Write)
    // }

    // pub fn from_slice(ctx: &WGPUContext, slice: &[T]) -> Self {
    //     // use wgpu::util::BufferInitDescriptor;
    //     let mem_align = MemAlign::new(slice.len());

    //     let inner = ctx.device().create_buffer(&wgpu::BufferDescriptor {
    //         label: None,
    //          /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
    //         // pub label: L,
    //         /// Size of a buffer.
    //         // pub size: BufferAddress,
    //         size: mem_align.byte_size as _,
    //         /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
    //         /// will panic.
    //         // pub usage: BufferUsage,
    //         usage: wgpu::BufferUsage::COPY_DST,
    //         /// Allows a buffer to be mapped immediately after they are made. It does not have to be [`BufferUsage::MAP_READ`] or
    //         /// [`BufferUsage::MAP_WRITE`], all buffers are allowed to be mapped at creation.
    //         // pub mapped_at_creation: bool,
    //         mapped_at_creation: true,
    //     });
    //     // Self {
    //     //     cpu: vec![],
    //     //     gpu:
    //     // }
    //     let mut self_ = Self {
    //         ctx: ctx.clone(),
    //         inner,
    //         len: 0,
    //         mem_align,
    //     };

    //     self_.extend_from_slice(slice);
    //     self_
    // }

    pub fn new_index(ctx: &WGPUContext, capacity: usize) -> Self {
        let mem_align = MemAlign::new(capacity);

        let usage = wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::INDEX;
        let inner = create_buffer(
            ctx,
            "index buffer",
            mem_align,
            //  wgpu::BufferUsage::INDEX,
            usage,
        );
        Self {
            usage,
            ctx: ctx.clone(),
            inner,
            // len: 0,
            mem_align,
        }
    }

    // pub fn len(&self) -> usize {
    //     self.len
    // }

    // pub fn extend_from_slice(&mut self, other: &[T]) {
    //     let new_len = self.len() + other.len();

    //     self.resize(new_len);

    //     unsafe {
    //         std::ptr::copy(other.as_ptr(), self.as_mut_ptr().offset(self.len() as _), other.len());
    //     }
    //     // self.ctx.queue().write_buffer(&self.inner, 0, as_u8_slice(other));

    //     self.len = new_len;
    // }

    pub fn resize(&mut self, capacity: usize) -> ResizeResult {
        if capacity <= self.capacity() {
            return ResizeResult::None;
        }

        let mem_align = MemAlign::<T>::new(capacity);
        // println!("resize to {:?}", mem_align.byte_size);

        // let inner = ctx.device().create_buffer(&wgpu::BufferDescriptor {

        // });

        // let inner = self.ctx.device().create_buffer(&wgpu::BufferDescriptor {
        //     label: None,
        //     size: mem_align.byte_size as _,

        //     mapped_at_creation: true,
        // });
        let inner = create_buffer(&self.ctx, "vertex buffer", mem_align, self.usage);

        // let inner = self.device.new_mem(
        //     mem_align,
        //     metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        // );
        // unsafe {
        //     std::ptr::copy(
        //         self.as_ptr(),
        //         // inner.contents() as *mut T,
        //         inner.slice(..).get_mapped_range_mut().as_mut_ptr() as *mut T,
        //         self.len(),
        //     );
        // }
        // self.ctx.queue.write_buffer(self, offset, data)
        self.mem_align = mem_align;
        self.inner.destroy();
        self.inner = inner;

        return ResizeResult::Resized;
    }

    // pub fn iter(&self) -> WGPUVecIterator<'_, T> {
    //     WGPUVecIterator::new(self)
    // }

    pub fn capacity(&self) -> usize {
        self.mem_align.capacity
    }

    // pub fn view_mut<'a>(&'a self) -> wgpu::BufferViewMut<'a> {
    //     self.inner.slice(..).get_mapped_range_mut()
    // }

    // pub fn upload(&mut self) {
    //     // self.gpu.destroy()
    //     todo!()
    // }

    // #[inline]
    // pub fn as_slice<S: std::ops::RangeBounds<wgpu::BufferAddress>>(&self, bounds: S) -> wgpu::BufferSlice {
    //     self.inner.slice(bounds)
    // }

    // pub fn slice(&self) -> wgpu::BufferSlice {
    //     self.inner.slice(..)
    // }

    // pub fn clear(&mut self) {
    //     self.len = 0;
    // }

    // #[inline]
    // pub fn as_ptr(&self) -> *const T {
    //     // self.inner.slice(bounds)
    //     self.slice().get_mapped_range().as_ptr() as *const T
    // }

    // #[inline]
    // fn as_mut_ptr(&mut self) -> *mut T {
    //     // self.inner.slice(bounds)
    //     // self.slice().get_mapped_range()
    //     // todo!()
    //     self.slice().get_mapped_range_mut().as_mut_ptr() as *mut T
    // }

    // // pub fn as_slice<'a>(&'a self) -> wgpu::BufferSlice<'a> {
    // //     // self.gpu.slice(0..0)
    // //     todo!()
    // // }

    // // pub fn as_mut_slice<'a>(&'a mut self) -> wgpu::BufferMutSlice<'a> {
    // //     todo!()
    // // }

    // fn element_byte_size() -> usize {
    //     std::mem::size_of::<T>()
    // }
}

// impl<T: Copy> std::ops::Index<usize> for WGPUVec<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         let view = self.slice().get_mapped_range();
//         assert!(self.capacity() * Self::element_byte_size() == view.len());
//         // let z = z.len();
//         let slice = unsafe { std::slice::from_raw_parts(view.as_ptr() as *const T, self.capacity()) };
//         &slice[index]
//     }
// }

impl<T: Copy> Drop for WGPUVec<T> {
    fn drop(&mut self) {
        self.inner.destroy()
    }
}

pub trait VecExt {
    fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize;
}

impl VecExt for Vec<u32> {
    fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize {
        let mut added = 0;
        for index in 1..(count - 1) {
            self.extend_from_slice(&[start, start + index, start + index + 1]);
            added += 3;
        }

        added
    }
}

// impl WGPUVec<u32> {
//     pub fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize {
//         let mut added = 0;
//         for index in 1..(count - 1) {
//             self.extend_from_slice(&[start, start + index, start + index + 1]);
//             added += 3;
//         }

//         added
//     }
// }

impl<T: Copy> AsRef<wgpu::Buffer> for WGPUVec<T> {
    fn as_ref(&self) -> &wgpu::Buffer {
        &self.inner
    }
}

mod tests {
    use super::{
        WGPUContext,
        WGPUInstance,
        WGPUVec,
    };

    // async fn async_vec_test() {
    //     let instance = WGPUInstance::new().await.unwrap();

    //     let context = WGPUContext::new(instance).await.unwrap();
    //     let mut v: WGPUVec<u32> = WGPUVec::new_vertex(&context, 10);
    //     v.extend_from_slice(&[10, 12]);

    //     // assert!(v.iter().collect() == )
    //     for e in v.iter() {
    //         println!("{:?}", e);
    //     }
    // }

    // #[test]
    // fn vec_test() {
    //     pollster::block_on(async_vec_test());
    // }
}
