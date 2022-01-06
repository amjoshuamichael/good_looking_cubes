// This file constructs the full shader files out of component parts. For example, the main voxel
// shader needs not only the main ray tracing code, but also some vector utility functions, functions
// to read from the world map, and several more functions that are contained in separate files.


pub static VERTEX_CANVAS: &str =
    concat!(
        include_str!("vertex_struct.wgsl"),
        include_str!("vertex_canvas.wgsl"),
    );

pub static VOXEL_RENDER: &str =
    concat!(
        include_str!("vertex_struct.wgsl"),
        include_str!("random.wgsl"),
        include_str!("vector_utils.wgsl"),
        include_str!("index_world.wgsl"),
        include_str!("plane_intersection.wgsl"),
        include_str!("voxel_render.wgsl"),
    );