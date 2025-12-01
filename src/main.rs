mod chunk;
mod render;
mod mesher;

use chunk::Chunk;
use chunk::chunk_definitions::Function;

use mesher::mesher_definitions::north_face;

use kiss3d::window::Window;
use nalgebra::Point3;

#[kiss3d::main]
async fn main() 
{
    chunk();

    let mut window = Window::new("Voxel-Renderer");
    window.set_background_color(0.0, 0.784, 1.0);

    let a = Point3::new(-1.0, -1.0, 0.0);
    let b = Point3::new(1.0, -1.0, 0.0);
    let c = Point3::new(0.0, 1.0, 0.0);

    let triangles = vec![a, b, c];

    render::build_mesh(&mut window, triangles);

    while window.render().await{}
}

#[allow(dead_code)]
fn chunk() 
{
    north_face();

    let mut chunk: Chunk = Chunk::new();

    chunk.generate(Function::Cube);

    chunk.array.set(0, 0, 0, 0);

    println!("Hello, chunk! You value is {}", chunk.array.get(0, 0, 0));
    println!("Hello, chunk! You value is {}", chunk.array.get(1, 1, 1));
}
