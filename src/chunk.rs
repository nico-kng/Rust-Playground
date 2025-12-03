//! # Module
//! ## Description:
//! This is a module.

pub mod chunk_generation;
pub mod chunk_definitions;

use chunk_definitions::Function;
use chunk_definitions::CHUNK_SIZE;

/// # Struct
/// ## Description
/// This is a struct.
#[allow(dead_code)]
pub struct ChunkArray<T>
{
    pub data: [T; CHUNK_SIZE],
}

#[allow(dead_code)]
impl<T: Copy + Default> ChunkArray<T>
{
    /// # Function
    /// ## Description
    /// This is a function.
    pub fn new() -> Self
    {
        Self
        {
            data: [T::default(); CHUNK_SIZE]
        }
    }

    /// # Function
    /// ## Description
    /// This is a function.
    fn index(x: usize, y: usize, z: usize) -> usize
    {
        if x < 8 && y < 8 && z < 8
        {
            (x << 6) | (y << 3) | z
        }
        else 
        {
            CHUNK_SIZE - 1  
        }
    }

    /// # Function
    /// ## Description
    /// This is a function.
    pub fn get(&self, x: usize, y: usize, z: usize) -> T
    {
        self.data[Self::index(x, y, z)]
    }
    
    /// # Function
    /// ## Description
    /// This is a function.
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T)
    {
        self.data[Self::index(x, y, z)] = value;
    }
}

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