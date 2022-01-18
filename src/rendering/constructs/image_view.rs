use gfx_hal::format::{Format, Swizzle};
use gfx_hal::image::{ SubresourceRange, ViewKind };
use gfx_hal::prelude::*;

pub unsafe fn create_image_view<B: gfx_hal::Backend>(
    device: &B::Device,
    image: &B::Image,
    format: Format,
) -> B::ImageView {
    device
        .create_image_view(
            image,
            ViewKind::D2,
            format,
            Swizzle::default(),
            SubresourceRange::default(),
        )
        .expect("failed to create image view")
}