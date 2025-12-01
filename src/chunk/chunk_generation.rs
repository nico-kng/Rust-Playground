//! # Submodule
//! ## Description:
//! This is a submodule.

use crate::chunk::chunk_array::ChunkArray;

/// # Function
/// ## Description
/// This is a function.
#[allow(dead_code)]
pub fn generate_cube(chunk: &mut ChunkArray<u8>)
{
    chunk.data.fill(1);
}