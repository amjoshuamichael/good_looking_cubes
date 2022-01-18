use std::ops::Range;
use gfx_hal::pso::ShaderStageFlags;
use bytemuck::Pod;

/// A small unit of data that is shared between both the CPU and GPU.
pub struct DataBuffer<T> {
    pub data: T,
    pub shader_stage: ShaderStageFlags,
}

impl<T: Default + Pod> DataBuffer<T> {
    pub fn new(shader_stage: ShaderStageFlags) -> Self {
        Self {
            data: T::default(),
            shader_stage,
        }
    }

    pub fn size() -> u32 {
        std::mem::size_of::<T>() as u32 * 4
    }

    pub fn layout(&self) -> (ShaderStageFlags, Range<u32>) {
        (self.shader_stage, 0..Self::size())
    }

    pub unsafe fn bytes(&self) -> &[u32] {
        let size_in_bytes = std::mem::size_of::<T>();
        let size_in_u32s = size_in_bytes / std::mem::size_of::<u32>();
        let start_ptr = &self.data as *const T as *const u32;
        std::slice::from_raw_parts(start_ptr, size_in_u32s)
    }
}