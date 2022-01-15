use crate::rendering::data_buffer::*;
use rand::prelude::*;
use rand::seq::SliceRandom;
use super::size;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct WorldData {
    pub data: [u32; (size * size * size) as usize],
}

impl DataForBuffer for WorldData {
    fn new() -> Self {
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

        let mut output = [0; (size * size * size) as usize];

        let mut rng = thread_rng();

        // for _ in 1..80 {
        //     let color: u32 = *colors.choose(&mut rng).unwrap();
        //     let place = rng.gen_range(0..512);
        //
        //     output[place] = color;
        // // }
        //
        for x in 0..size {
            for z in 0..size {
                let color: u32 = *colors.choose(&mut rng).unwrap();

                output[(x + z * size * size) as usize] = color;
            }
        }

        for y in 0..size {
            for z in 0..size {
                let color: u32 = *colors.choose(&mut rng).unwrap();

                output[(y * size + z * size * size) as usize] = color;
            }
        }

        WorldData {
            data: output,
        }
    }
}