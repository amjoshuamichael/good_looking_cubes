// This file constructs the full shader files out of component parts. For example, the main voxel
// shader needs not only the main ray tracing code, but also some vector utility functions, functions
// to read from the world map, and several more functions that are contained in separate files.


pub static VERTEX_CANVAS: &str =
    concat!(
        include_str!("version_header.glsl"),
        include_str!("canvas.vert"),
    );

pub static VOXEL_RENDER: &str =
    concat!(
        include_str!("version_header.glsl"),
        include_str!("pc_buffer.glsl"),
        include_str!("vector_utils.glsl"),
        include_str!("random.glsl"),
        include_str!("index_world.glsl"),
        include_str!("plane_intersection.glsl"),
        include_str!("voxel_render.frag"),
    );

pub static POST_PROCESSING: &str =
    concat!(
        include_str!("version_header.glsl"),
        include_str!("post_processing.glsl"),
    );

pub static WORLD_DRAW: &str =
    concat!(
        include_str!("version_header.glsl"),
        include_str!("world_draw.glsl"),
    );
