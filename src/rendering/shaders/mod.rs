// This file constructs the full shader files out of component parts. For example, the main voxel
// shader needs not only the main ray tracing code, but also some vector utility functions, functions
// to read from the world map, and several more functions that are contained in separate files.

pub const VERTEX_CANVAS: &str = concat!(
    include_str!("version_header.glsl"),
    include_str!("canvas.vert"),
);

pub const VOXEL_RENDER: &str = concat!(
    include_str!("version_header.glsl"),
    include_str!("pc_buffer.glsl"),
    include_str!("world_buffer.glsl"),
    include_str!("vector_utils.glsl"),
    include_str!("index_world.glsl"),
    include_str!("extract_color.glsl"),
    include_str!("hit_in_direction.glsl"),
    include_str!("random.glsl"),
    include_str!("voxel_render.frag"),
);

pub const POST_PROCESSING: &str = concat!(
    include_str!("version_header.glsl"),
    include_str!("pc_buffer.glsl"),
    include_str!("vector_utils.glsl"),
    include_str!("post_processing.glsl"),
);
