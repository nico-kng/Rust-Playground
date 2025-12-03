//! # Module
//! ## Description:
//! This is a module.
//! 
//! ### LIST:
//! #### mesher_dumb:
//! 1. generate mesh into chunk array (example is a chunk filled with ones)
//! 2. iterate over chunk array for every voxel
//! 3. build all 6 quads for the current voxel
//! 4. build 2 triangles per quad for rendering
//! 
//! #### mesher_culled:
//! 1. generate mesh into chunk array (example is a chunk filled with ones)
//! 2. iterate over chunk array for every voxel
//! 3. build only the quads that faces non-solid blocks
//! 4. build 2 triangles per quad for rendering
//! - optimize the algorithm with bitwise operations 
//! 
//! #### mesher_greedy:
//! 1. generate mesh into chunk array (example is a chunk filled with ones)
//! 2. iterate over all 6 directions (x+, x-, y+, y-, z+, z-)
//! 3. iterate over all planes in this direction
//! 4. build a bitmask for faces using culled meshing
//! 5. search all merged quads from every plane bitmask and all 6 directions
//! 6. build 2 triangles per quad for rendering
//! - optimize the algorithm with bitwise operations

mod mesher_dumb;
pub mod mesher_definitions;

use crate::mesher::mesher_definitions::FACES;

/// # Function
/// ## Description
/// This is a function.
#[allow(dead_code)]
pub fn create_vertices
(
    verts: &mut Vec<[f32; 3]>, 
    x: &usize, 
    y: &usize, 
    z: &usize, 
    size: &usize, 
    face: FACES
)
{
    let x = *x as f32;
    let y = *y as f32;
    let z = *z as f32;
    let size = *size as f32;

    let mut _quad: [[f32; 3]; 4] = [[0.0; 3]; 4];

    match face 
    {
        FACES::NORTH =>
        {
            _quad = 
            [
                [1.0 * size + x, 0.0 * size + y, 1.0 * size + z],
                [1.0 * size + x, 0.0 * size + y, 0.0 * size + z],
                [1.0 * size + x, 1.0 * size + y, 0.0 * size + z],
                [1.0 * size + x, 1.0 * size + y, 1.0 * size + z],
            ];
        },
        FACES::SOUTH =>
        {
            _quad = 
            [
                [0.0 * size + x, 0.0 * size + y, 0.0 * size + z],
                [0.0 * size + x, 0.0 * size + y, 1.0 * size + z],
                [0.0 * size + x, 1.0 * size + y, 1.0 * size + z],
                [0.0 * size + x, 1.0 * size + y, 0.0 * size + z],
            ];
        },
        FACES::EAST =>
        { 
            _quad = 
            [
                [0.0 * size + x, 0.0 * size + y, 1.0 * size + z],
                [1.0 * size + x, 0.0 * size + y, 1.0 * size + z],
                [1.0 * size + x, 1.0 * size + y, 1.0 * size + z],
                [0.0 * size + x, 1.0 * size + y, 1.0 * size + z],
            ];
        },
        FACES::WEST => 
        { 
            _quad = 
            [
                [1.0 * size + x, 0.0 * size + y, 0.0 * size + z],
                [0.0 * size + x, 0.0 * size + y, 0.0 * size + z],
                [0.0 * size + x, 1.0 * size + y, 0.0 * size + z],
                [1.0 * size + x, 1.0 * size + y, 0.0 * size + z],
            ];
        },
        FACES::TOP => 
        { 
            _quad = 
            [
                [0.0 * size + x, 1.0 * size + y, 0.0 * size + z],
                [0.0 * size + x, 1.0 * size + y, 1.0 * size + z],
                [1.0 * size + x, 1.0 * size + y, 1.0 * size + z],
                [1.0 * size + x, 1.0 * size + y, 0.0 * size + z],
            ];
        },
        FACES::BOTTOM =>
        {
            _quad = 
            [
                [0.0 * size + x, 0.0 * size + y, 1.0 * size + z],
                [0.0 * size + x, 0.0 * size + y, 0.0 * size + z],
                [1.0 * size + x, 0.0 * size + y, 0.0 * size + z],
                [1.0 * size + x, 0.0 * size + y, 1.0 * size + z],
            ];
        },
    }

    verts.extend([_quad[0], _quad[1], _quad[2], _quad[0], _quad[2], _quad[3]]);

}