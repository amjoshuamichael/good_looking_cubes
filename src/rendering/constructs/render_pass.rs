use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::image::Layout;
use gfx_hal::pass::{
    Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc
};

pub unsafe fn create_render_pass<B: gfx_hal::Backend>(
    device: &B::Device,
    color_format: Format,
) -> B::RenderPass {
    device
        .create_render_pass(&[
            Attachment {
                format: Some(color_format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::General..Layout::General,
            }
        ], &[
            SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            }
        ], &[])
        .expect("Out of memory")
}
