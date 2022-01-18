use std::fs::File;
use bytemuck;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct CameraData {
    pub pos: [f32; 4],
    pub dir: [f32; 4],

    pub palette: [[f32; 4]; 256],

    pub text_to_show: [u32; 256],
}

impl Default for CameraData {
    fn default() -> Self {
        let palette = {
            let mut palette = [[0.0; 4]; 256];

            let decoder = png::Decoder::new(File::open("assets/palettes/monu16.png").unwrap());
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

        CameraData {
            pos: [0.0, 0.0, 0.0, 0.0],
            dir: [0.0, 0.0, 0.0, 0.0],
            palette,
            text_to_show: [0; 256],
        }
    }
}