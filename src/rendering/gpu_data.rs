use std::fs::File;
use std::ops::Range;
use bytemuck;
use gfx_hal::pso::ShaderStageFlags;

/// Data that needs to be pushed to the gpu, that isn't world data or a texture.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct GPUData {
    pub pos: [f32; 4],
    pub dir: [f32; 4],

    pub palette: [[f32; 4]; 256],

    pub text_to_show: [u32; 256],

    pub contrast: f32,
    pub brightness: f32,
    pub exposure: f32,
    pub hue: f32,
    pub saturation: f32,
}

impl Default for GPUData {
    fn default() -> Self {
        let palette = {
            let mut palette = [[0.0; 4]; 256];

            let decoder = png::Decoder::new(File::open("assets/palettes/basic.png").unwrap());
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut buf).unwrap();
            let bytes = &buf[..info.buffer_size()];

            for c in 0..256 {
                let byte_index = c * 4;

                palette[c] = [
                    bytes[byte_index    ] as f32 / 256.0,
                    bytes[byte_index + 1] as f32 / 256.0,
                    bytes[byte_index + 2] as f32 / 256.0,
                    bytes[byte_index + 3] as f32 / 256.0,
                ]
            }

            palette
        };

        GPUData {
            pos: [0.0, 0.0, 0.0, 0.0],
            dir: [0.0, 0.0, 0.0, 0.0],
            palette,
            text_to_show: [0; 256],
            contrast: 0.0,
            brightness: 0.0,
            exposure: 1.0,
            hue: 0.0,
            saturation: 1.0,
        }
    }
}

impl GPUData {
    pub fn size() -> u32 {
        std::mem::size_of::<GPUData>() as u32 * 4
    }

    pub fn layout(&self) -> (ShaderStageFlags, Range<u32>) {
        (ShaderStageFlags::ALL, 0..Self::size())
    }

    pub unsafe fn bytes(&self) -> &[u32] {
        let size_in_bytes = std::mem::size_of::<GPUData>();
        let size_in_u32s = size_in_bytes / std::mem::size_of::<u32>();
        let start_ptr = self as *const GPUData as *const u32;
        std::slice::from_raw_parts(start_ptr, size_in_u32s)
    }
}