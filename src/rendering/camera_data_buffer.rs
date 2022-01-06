use super::data_buffer::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraData {
    pub pos: [f32; 3],
    padding_1: u32,
    pub dir: [f32; 3],
    padding_2: u32,
    pub lpos: [f32; 3],
    padding_3: u32,
    pub ldir: [f32; 3],
    padding_4: u32,
}

impl DataForBuffer for CameraData {
    fn create() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0],
            dir: [0.0, 0.0, 1.0],
            lpos: [0.0, 0.0, 0.0],
            ldir: [1.0, -1.0, 1.0],
            padding_1: 0, padding_2: 0, padding_3: 0, padding_4: 0,
        }
    }
}