mod chunk;
mod render;
mod mesher;

use chunk::Chunk;
use chunk::chunk_definitions::Function;

use mesher::*;
use mesher::mesher_definitions::FACES;

use kiss3d::window::Window;

/// MAIN FUNCTION
#[kiss3d::main]
async fn main() 
{
    let mut window = Window::new("Voxel-Renderer");
    window.set_background_color(0.0, 0.784, 1.0);

    let mut chunk: Chunk = Chunk::new();
    chunk.generate(Function::Cube);

    let mut verts: Vec<[f32; 3]> = Vec::new();

    let x: usize = 0;
    let y: usize = 0;
    let z: usize = 0;
    let size: usize = 1;

    create_vertices(&mut verts, &x, &y, &z, &size, FACES::NORTH);
    create_vertices(&mut verts, &x, &y, &z, &size, FACES::SOUTH);
    create_vertices(&mut verts, &x, &y, &z, &size, FACES::EAST);
    create_vertices(&mut verts, &x, &y, &z, &size, FACES::WEST);
    create_vertices(&mut verts, &x, &y, &z, &size, FACES::TOP);
    create_vertices(&mut verts, &x, &y, &z, &size, FACES::BOTTOM);

    render::build_mesh(&mut window, verts);

    while window.render().await{}

}
