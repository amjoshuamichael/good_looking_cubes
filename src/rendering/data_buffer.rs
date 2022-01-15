use std::ops::Range;
use gfx_hal::pso::ShaderStageFlags;
use bytemuck::Pod;

/// Data that is shared between both the CPU and GPU.
pub struct DataBuffer<T> {
    pub data: T,
    pub shader_stage: ShaderStageFlags,
}

pub trait DataForBuffer {
    fn new() -> Self;
}

impl<T: DataForBuffer + Pod> DataBuffer<T> {
    pub fn new(shader_stage: ShaderStageFlags) -> Self {
        Self {
            data: T::new(),
            shader_stage,
        }
    }

    pub fn size() -> u32 {
        std::mem::size_of::<T>() as u32 * 4
    }

    pub fn layout(&self) -> (ShaderStageFlags, Range<u32>) {
        println!("{}", Self::size());
        (self.shader_stage, 0..Self::size())
    }

    pub unsafe fn bytes(&self) -> &[u32] {
        let size_in_bytes = std::mem::size_of::<T>();
        let size_in_u32s = size_in_bytes / std::mem::size_of::<u32>();
        let start_ptr = &self.data as *const T as *const u32;
        std::slice::from_raw_parts(start_ptr, size_in_u32s)
    }

    pub unsafe fn bytes_8(&self) -> &[u8] {
        let size_in_bytes = std::mem::size_of::<T>();
        let size_in_u8s = size_in_bytes / std::mem::size_of::<u8>();
        let start_ptr = &self.data as *const T as *const u8;
        std::slice::from_raw_parts(start_ptr, size_in_u8s)
    }
}