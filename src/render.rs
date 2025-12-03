//! # Module
//! ## Description:
//! This is a module.

use kiss3d::light::Light;
use kiss3d::procedural::RenderMesh;
use kiss3d::resource::GpuMesh;
use kiss3d::window::Window;
use nalgebra::Point3;
use nalgebra::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

/// # Function
/// ## Description
/// This is a function.
#[allow(dead_code)]
fn create_points(triangles: &mut Vec<Point3<f32>>, vertices: &Vec<[f32; 3]>)
{
    for vertex in vertices 
    {
        triangles.push(Point3::new(vertex[0], vertex[1], vertex[2]));
    }
}

/// # Function
/// ## Description
/// This is a function.
#[allow(dead_code)]
pub fn build_mesh(window: &mut Window, vertices: Vec<[f32; 3]>)
{
    let mut triangles: Vec<Point3<f32>> = Vec::new();
    create_points(&mut triangles, &vertices);
    let render_mesh = RenderMesh::new(triangles, None, None, None);
    let gpu_mesh = GpuMesh::from_render_mesh(render_mesh, false);
    let mesh = Rc::new(RefCell::new(gpu_mesh));
    let mut c = window.add_mesh(mesh, Vector3::new(1.0, 1.0, 1.0));
    c.set_color(0.0, 0.0, 0.0);
    c.set_lines_width(1.0);
    c.set_surface_rendering_activation(false);
    window.set_light(Light::StickToCamera);
}