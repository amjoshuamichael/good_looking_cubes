use gfx_hal::{
    prelude::*,
    pso::{
        ImageDescriptorType, ShaderStageFlags, DescriptorSetLayoutBinding, DescriptorType,
        DescriptorRangeDesc, DescriptorPoolCreateFlags, Descriptor, DescriptorSetWrite
    },
    device::Device,
    image::{
        Filter, SamplerDesc, WrapMode, Layout
    },
};

#[inline]
pub unsafe fn create_image_bindings<B: gfx_hal::Backend>(
    device: &B::Device,
    image_view: &B::ImageView,
) -> (B::DescriptorSetLayout, B::DescriptorSet, B::Sampler) {
    let set_layout =
        device.create_descriptor_set_layout(
            &[
                DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: DescriptorType::Image {
                        ty: ImageDescriptorType::Sampled {
                            with_sampler: true
                        }
                    },
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
            &[],
        )
            .expect("Ran out of memory creating descriptor set layout");

    let mut desc_pool = device.create_descriptor_pool(
        1, // sets
        &[
            DescriptorRangeDesc {
                ty: DescriptorType::Image {
                    ty: ImageDescriptorType::Sampled {
                        with_sampler: true
                    }
                },
                count: 1,
            },
            DescriptorRangeDesc {
                ty: DescriptorType::Sampler,
                count: 1,
            },
        ],
        DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
    )
        .expect("Can't create descriptor pool");

    let desc_set = desc_pool.allocate_set(&set_layout).unwrap();

    let sampler =
        device
            .create_sampler(&SamplerDesc::new(Filter::Nearest, WrapMode::Clamp))
            .unwrap();

    device.write_descriptor_sets(vec![
        DescriptorSetWrite {
            set: &desc_set,
            binding: 0,
            array_offset: 0,
            descriptors: Some(Descriptor::Image(image_view, Layout::General)),
        },
        DescriptorSetWrite {
            set: &desc_set,
            binding: 1,
            array_offset: 0,
            descriptors: Some(Descriptor::Sampler(&sampler)),
        },
    ]);

    (set_layout, desc_set, sampler)
}