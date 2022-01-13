use super::data_buffer::DataForBuffer;
use bytemuck;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct CameraData {
    pub pos: [f32; 4],
    pub dir: [f32; 4],
}

impl DataForBuffer for CameraData {
    fn new() -> Self {
        Self {
            pos: [1.0, 0.0, -1.0, 1.0],
            dir: [1.0, 0.0, 0.0, 1.0]
        }
    }
}