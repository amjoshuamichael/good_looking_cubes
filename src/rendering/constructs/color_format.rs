use gfx_hal::adapter::Adapter;
use gfx_hal::prelude::*;
use gfx_hal::format::Format;

pub fn find_color_format<B: gfx_hal::Backend, Closure: FnMut(&Format) -> bool>(
    surface: &B::Surface,
    adapter: &Adapter<B>,
    closure: Closure,
) -> Format {
    let supported_formats = surface
        .supported_formats(&adapter.physical_device).unwrap_or_default();

    let default_format = *supported_formats.get(0).unwrap_or(&Format::Rgba8Srgb);

    supported_formats.into_iter()
        .find(closure)
        .unwrap_or(default_format)
}