//! # Module
//! ## Description:
//! This is a module.

pub mod chunk_array;
pub mod chunk_generation;
pub mod chunk_definitions;
mod chunk_settings;

use chunk_array::ChunkArray;
use chunk_definitions::Function;

/// # Struct
/// ## Description
/// This is a struct.
#[allow(dead_code)]
pub struct Chunk
{
    pub array: ChunkArray<u8>,
}

#[allow(dead_code)]
impl Chunk
{   
    /// # Function
    /// ## Description
    /// This is a function.
    pub fn new() -> Self
    {
        Self
        {
            array: ChunkArray::new(),
        }
    }

    /// # Function
    /// ## Description
    /// This is a function.
    pub fn generate(&mut self, mode: Function)
    {
        match mode
        {
            Function::Cube =>
            {
                chunk_generation::generate_cube(&mut self.array);
            }
        }
    }
}