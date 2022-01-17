use gfx_hal::buffer::SubRange;
use gfx_hal::prelude::*;
use gfx_hal::pso::*;

pub unsafe fn create_buffer_bindings<B: gfx_hal::Backend>(
    device: &B::Device,
    buffer: &B::Buffer,
) -> (B::DescriptorSetLayout, B::DescriptorSet) {
    let set_layout =
        device.create_descriptor_set_layout(
            &[
                DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: DescriptorType::Buffer {
                        ty: BufferDescriptorType::Uniform,
                        format: BufferDescriptorFormat::Structured {
                            dynamic_offset: false,
                        },
                    },
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
            &[],
        )
            .expect("Can't create descriptor set layout");

    let mut desc_pool = device.create_descriptor_pool(
        1, // sets
        &[
            DescriptorRangeDesc {
                ty: DescriptorType::Buffer {
                    ty: BufferDescriptorType::Uniform,
                    format: BufferDescriptorFormat::Structured {
                        dynamic_offset: false,
                    },
                },
                count: 1,
            },
        ],
        gfx_hal::pso::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
    )
        .expect("Can't create descriptor pool");

    let desc_set = desc_pool
        .allocate_set(&set_layout)
        .expect("unable to allocate set layout for description pool");

    device.write_descriptor_sets(vec![
        DescriptorSetWrite {
            set: &desc_set,
            binding: 0,
            array_offset: 0,
            descriptors: Some(Descriptor::Buffer(
                buffer,
                SubRange {
                    offset: 0,
                    size: None,
                },
            )),
        },
    ]);

    (set_layout, desc_set)
}