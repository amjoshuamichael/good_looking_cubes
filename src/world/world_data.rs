use crate::rendering::data_buffer::DataForBuffer;
use crate::rendering::data_buffer::*;
use rand::prelude::*;
use rand::seq::SliceRandom;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WorldData {
    data: [u32; 512],
}

impl DataForBuffer for WorldData {
    fn create() -> Self {
        let colors: [u32; 8] = [
            0b00000000000000000000000010000000,
            0b10000000000000000000000010000000,
            0b00000000100000000000000010000000,
            0b00000000000000001000000010000000,
            0b10000000100000000000000010000000,
            0b00000000100000001000000010000000,
            0b10000000000000001000000010000000,
            0b10000000100000001000000010000000,
        ];

        let mut output = [0; 512];

        let mut rng = thread_rng();

        for _ in 1..100 {
            let color: u32 = *colors.choose(&mut rng).unwrap();
            let place = rng.gen_range(0..512);

            output[place] = color;
        }

        // for x in 0..8 {
        //     for z in 0..8 {
        //         let color: u32 = *colors.choose(&mut rng).unwrap();
        //
        //         output[x + z * 64] = color;
        //     }
        // }
        //
        // for y in 0..8 {
        //     for z in 0..8 {
        //         let color: u32 = *colors.choose(&mut rng).unwrap();
        //
        //         output[y * 8 + z * 64] = color;
        //     }
        // }

        WorldData {
            data: output,
        }
    }
}