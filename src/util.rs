use cgmath::Point3;
use cgmath::Vector3;
use cgmath::prelude::*;
use cgmath::Deg;

pub fn norm(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Vector3<f32> {
    let side1 = p2 - p1;
    let side2 = p3 - p1;
    let perp = side1.cross(side2);
    perp.normalize();
    perp
}