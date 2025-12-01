//! # Submodule
//! ## Description:
//! This is a submodule.

use nalgebra::{Point3, Vector3};

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

static _VERTICES_TABLE: [Point3<f32>; 8] = 
[
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(0.0, 0.0, 0.0),
];

#[allow(dead_code)]
pub fn north_face()
{
    // vert = pos * factor + offset
    let _point = _VERTICES_TABLE[0] * 2.0 + Vector3::new(2.0, 2.0, 2.0);
    println!("{}", _point);
}