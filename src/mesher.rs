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

/// # Enum
/// ## Description
/// This is an enum.
#[allow(dead_code)]
enum Mode 
{
    DUMB,
    CULLED,
    GREEDY,
}