use gfx_hal::adapter::Adapter;
use gfx_hal::format::Format;
use gfx_hal::prelude::*;

use gfx_hal::queue::QueueGroup;
use gfx_hal::window::Extent2D;

pub struct RenderInfo<B: gfx_hal::Backend> {
    pub instance: B::Instance,
    pub surface: B::Surface,
    pub device: B::Device,
    pub render_passes: Vec<B::RenderPass>,
    pub pipeline_layouts: Vec<B::PipelineLayout>,
    pub render_pipelines: Vec<B::GraphicsPipeline>,
    pub compute_pipelines: Vec<B::ComputePipeline>,
    pub image_views: Vec<B::ImageView>,
    pub description_sets: Vec<B::DescriptorSet>,
    pub samplers: Vec<B::Sampler>,
    pub buffers: Vec<B::Buffer>,
    pub buffer_views: Vec<B::BufferView>,
    pub command_pool: B::CommandPool,
    pub submission_complete_fence: B::Fence,
    pub rendering_complete_semaphore: B::Semaphore,
    pub surface_extent: Extent2D,
    pub adapter: Adapter<B>,
    pub queue_group: QueueGroup<B>,
    pub surface_color_format: Format,
    pub render_resolution: (u32, u32),
}

impl<B: gfx_hal::Backend> RenderInfo<B> {
    pub fn destroy_all(mut self) {
        unsafe {
            self.device.destroy_semaphore(self.rendering_complete_semaphore);
            self.device.destroy_fence(self.submission_complete_fence);
            for pipeline in self.render_pipelines {self.device.destroy_graphics_pipeline(pipeline); }
            for pipeline in self.compute_pipelines {self.device.destroy_compute_pipeline(pipeline); }
            for layout in self.pipeline_layouts {self.device.destroy_pipeline_layout(layout); }
            for pass in self.render_passes {self.device.destroy_render_pass(pass); }
            for view in self.image_views {self.device.destroy_image_view(view); }
            for sampler in self.samplers {self.device.destroy_sampler(sampler); }
            for buffer in self.buffers {self.device.destroy_buffer(buffer); }
            for view in self.buffer_views {self.device.destroy_buffer_view(view); }
            self.device.destroy_command_pool(self.command_pool);
            self.surface.unconfigure_swapchain(&self.device);
            self.instance.destroy_surface(self.surface);
        }
    }
}