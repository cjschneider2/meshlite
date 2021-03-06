use cgmath::Point3;
use cgmath::Vector3;
use cgmath::prelude::*;
use cgmath::Deg;
use cgmath::Rad;

/*
Range of the Dot Product of Two Unit Vectors

Dot     Angle
1.000	0 degrees
0.966	15 degrees
0.866	30 degrees
0.707	45 degrees
0.500	60 degrees
0.259	75 degrees
0.000	90 degrees
-0.259	105 by degrees
-0.500	120 degrees
-0.707	135 degrees
-0.866	150 degrees
-0.966	165 degrees
-1.000	180 degrees

Source: http://chortle.ccsu.edu/vectorlessons/vch09/vch09_6.html
 */

pub fn norm(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Vector3<f32> {
    let side1 = p2 - p1;
    let side2 = p3 - p1;
    let perp = side1.cross(side2);
    perp.normalize()
}

pub fn almost_eq(v1: Vector3<f32>, v2: Vector3<f32>) -> bool {
    (v1.x - v2.x).abs() <= 0.01 &&
        (v1.y - v2.y).abs() <= 0.01 &&
        (v1.z - v2.z).abs() <= 0.01
}

// Modifed from Reza Nourai's C# version: PointInTriangle
// https://blogs.msdn.microsoft.com/rezanour/2011/08/07/barycentric-coordinates-and-point-in-triangle-tests/
pub fn point_in_triangle(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>, p: Point3<f32>) -> bool {
    let u = b - a;
    let v = c - a;
    let w = p - a;
    let v_cross_w = v.cross(w);
    let v_cross_u = v.cross(u);
    if v_cross_w.dot(v_cross_u) < 0.0 {
        return false;
    }
    let u_cross_w = u.cross(w);
    let u_cross_v = u.cross(v);
    if u_cross_w.dot(u_cross_v) < 0.0 {
        return false;
    }
    let denom = u_cross_v.magnitude();
    let r = v_cross_w.magnitude() / denom;
    let t = u_cross_w.magnitude() / denom;
    r + t <= 1.0
}

// Modified from Cyranose's answer
// https://www.opengl.org/discussion_boards/showthread.php/159385-Deriving-angles-from-0-to-360-from-Dot-Product
pub fn angle360(a: Vector3<f32>, b: Vector3<f32>, direct: Vector3<f32>) -> f32 {
    let angle = Rad::acos(a.dot(b));
    let c = a.cross(b);
    if c.dot(direct) < 0.0 {
        180.0 + Deg::from(angle).0
    } else {
        Deg::from(angle).0
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum PointSide {
    Front,
    Back,
    Coincident
}

pub fn point_side_on_plane(pt: Point3<f32>, pt_on_plane: Point3<f32>, norm: Vector3<f32>) -> PointSide {
    let line = pt - pt_on_plane;
    let dot = line.dot(norm);
    if dot > 0.0 {
        PointSide::Front
    } else if dot < 0.0 {
        PointSide::Back
    } else {
        PointSide::Coincident
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SegmentPlaneIntersect {
    NoIntersection,
    Parallel,
    LiesIn,
    Intersection(Point3<f32>),
}

pub const SMALL_NUM : f32 = 0.00000001;

// Modfied from the C++ version intersect3D_SegmentPlane
// http://geomalgorithms.com/a05-_intersect-1.html
pub fn intersect_of_segment_and_plane(p0: Point3<f32>, p1: Point3<f32>, pt_on_plane: Point3<f32>, norm: Vector3<f32>) -> SegmentPlaneIntersect {
    let u = p1 - p0;
    let w = p0 - pt_on_plane;
    let d = norm.dot(u);
    let n = -norm.dot(w);
    if d.abs() < SMALL_NUM {
        if n == 0.0 {
            return SegmentPlaneIntersect::LiesIn;
        }
        return SegmentPlaneIntersect::Parallel;
    }
    let s_i = n / d;
    if s_i < 0.0 || s_i > 1.0 || s_i.is_nan() || s_i.is_infinite() {
        return SegmentPlaneIntersect::NoIntersection;
    }
    SegmentPlaneIntersect::Intersection(p0 + (s_i * u))
}

// Modified from intersectRayWithSquare
// https://stackoverflow.com/questions/21114796/3d-ray-quad-intersection-test-in-java
pub fn is_segment_and_quad_intersect(p0: Point3<f32>, p1: Point3<f32>, quad: &Vec<Point3<f32>>) -> bool {
    let r1 = p0;
    let r2 = p1;
    let s1 = quad[0];
    let s2 = quad[1];
    let s3 = quad[2];
    let ds21 = s2 - s1;
    let ds31 = s3 - s1;
    let n = ds21.cross(ds31);
    let dr = r1 - r2;
    let ndotdr = n.dot(dr);
    if ndotdr.abs() < SMALL_NUM {
        return false;
    }
    let t = -n.dot(r1 - s1) / ndotdr;
    let m = r1 + (dr * t);
    let dms1 = m - s1;
    let u = dms1.dot(ds21);
    let v = dms1.dot(ds31);
    u >= 0.0 && u <= ds21.dot(ds21) && v >= 0.0 && v <= ds31.dot(ds31)
}

pub fn is_two_quads_intersect(first_quad: &Vec<Point3<f32>>, second_quad: &Vec<Point3<f32>>) -> bool {
    for i in 0..second_quad.len() {
        if is_segment_and_quad_intersect(second_quad[i], second_quad[(i + 1) % second_quad.len()], first_quad) {
            return true;
        }
    }
    for i in 0..first_quad.len() {
        if is_segment_and_quad_intersect(first_quad[i], first_quad[(i + 1) % first_quad.len()], second_quad) {
            return true;
        }
    }
    false
}

pub fn is_point_on_segment(point: Point3<f32>, seg_begin: Point3<f32>, seg_end: Point3<f32>) -> bool {
    let v = seg_end - seg_begin;
    let w = point - seg_begin;
    let w_dot_v = w.dot(v);
    if w_dot_v <= 0.0 {
        return false;
    }
    let v_dot_v = v.dot(v);
    if v_dot_v <= w_dot_v {
        return false;
    }
    let t = seg_begin + (v * (w_dot_v / v_dot_v));
    let dist = t.distance(point);
    dist <= 0.00001
}

pub fn is_valid_norm(norm: Vector3<f32>) -> bool {
    !norm.x.is_nan() && !norm.y.is_nan() && !norm.z.is_nan()
}

pub fn pick_base_plane_norm(directs: Vec<Vector3<f32>>, positions: Vec<Point3<f32>>, weights: Vec<f32>) -> Option<Vector3<f32>> {
    if directs.len() <= 1 {
        None
    } else if directs.len() <= 2 {
        // >=15 degrees && <= 165 degrees
        if directs[0].dot(directs[1]).abs() < 0.966 {
            return Some(directs[0].cross(directs[1]).normalize())
        }
        None
    } else if directs.len() <= 3 {
        let norm = norm(positions[0], positions[1], positions[2]);
        if is_valid_norm(norm) {
            return Some(norm.normalize());
        }
        // >=15 degrees && <= 165 degrees
        if directs[0].dot(directs[1]).abs() < 0.966 {
            return Some(directs[0].cross(directs[1]).normalize())
        } else if directs[1].dot(directs[2]).abs() < 0.966 {
            return Some(directs[1].cross(directs[2]).normalize())
        } else if directs[2].dot(directs[0]).abs() < 0.966 {
            return Some(directs[2].cross(directs[0]).normalize())
        } else {
            None
        }
    } else {
        let mut weighted_indices : Vec<(usize, usize)> = Vec::new();
        for i in 0..weights.len() {
            weighted_indices.push((i, (weights[i] * 100.0) as usize));
        }
        weighted_indices.sort_by(|a, b| b.1.cmp(&a.1));
        let i0 = weighted_indices[0].0;
        let i1 = weighted_indices[1].0;
        let i2 = weighted_indices[2].0;
        let norm = norm(positions[i0], positions[i1], positions[i2]);
        if is_valid_norm(norm) {
            return Some(norm.normalize());
        }
        // >=15 degrees && <= 165 degrees
        if directs[i0].dot(directs[i1]).abs() < 0.966 {
            return Some(directs[i0].cross(directs[i1]).normalize())
        } else if directs[i1].dot(directs[i2]).abs() < 0.966 {
            return Some(directs[i1].cross(directs[i2]).normalize())
        } else if directs[i2].dot(directs[i0]).abs() < 0.966 {
            return Some(directs[i2].cross(directs[i0]).normalize())
        } else {
            None
        }
    }
}

pub fn world_perp(direct: Vector3<f32>) -> Vector3<f32> {
    const WORLD_Y_AXIS : Vector3<f32> = Vector3 {x: 0.0, y: 1.0, z: 0.0};
    const WORLD_X_AXIS : Vector3<f32> = Vector3 {x: 1.0, y: 0.0, z: 0.0};
    if direct.dot(WORLD_X_AXIS).abs() > 0.707 {
        // horizontal
        //println!("switch to WORLD_Y_AXIS");
        direct.cross(WORLD_Y_AXIS)
    } else {
        // vertical
        //println!("switch to WORLD_X_AXIS");
        direct.cross(WORLD_X_AXIS)
    }
}

pub fn calculate_deform_position(vert_position: Point3<f32>, vert_ray: Vector3<f32>, deform_norm: Vector3<f32>, deform_factor: f32) -> Point3<f32> {
    let revised_norm = if vert_ray.dot(deform_norm) < 0.0 {
        -deform_norm
    } else {
        deform_norm
    };
    let proj = vert_ray.project_on(revised_norm);
    let scaled_proj = proj * deform_factor;
    let scaled_vert_ray = Vector3 {x:vert_position.x, y:vert_position.y, z:vert_position.z} + 
            (scaled_proj - proj);
    Point3 {x: scaled_vert_ray.x, y: scaled_vert_ray.y, z: scaled_vert_ray.z}
}

pub fn make_quad(position: Point3<f32>, direct: Vector3<f32>, radius: f32, base_norm: Vector3<f32>) -> Vec<Point3<f32>> {
    let direct_normalized = direct.normalize();
    let base_norm_normalized = base_norm.normalize();
    let dot = direct_normalized.dot(base_norm);
    //println!("make_quad begin dot:{:?} direct:{:?}(normalize:{:?}) base_norm:{:?}(normalize:{:?})", 
    //    dot, direct, direct_normalized, base_norm, base_norm_normalized);
    let oriented_base_norm = {
        if dot > 0.0 {
            base_norm_normalized
        } else {
            //println!("base_norm reversed");
            -base_norm_normalized
        }
    };
    let u = {
        if direct_normalized.dot(oriented_base_norm).abs() > 0.707 {
            // same direction with < 45 deg
            //println!("< 45 deg");
            let switched_base_norm = world_perp(oriented_base_norm);
            direct_normalized.cross(switched_base_norm)
        } else {
            // same direction with >= 45 deg
            //println!(">= 45 deg");
            direct_normalized.cross(oriented_base_norm)
        }
    };
    let v = u.cross(direct);
    let u = u.normalize() * radius;
    let v = v.normalize() * radius;
    let origin = position + direct * radius;
    let f = vec![origin - u - v,
        origin + u - v,
        origin + u + v,
        origin - u + v];
    //println!("u:{:?} v:{:?}", u, v);
    //println!("f:{:?}", f);
    //println!("make_quad end");
    f
}

pub fn pick_most_not_obvious_vertex(vertices: Vec<Point3<f32>>) -> usize {
    if vertices.len() <= 1 {
        return 0;
    }
    let mut choosen_index = 0;
    let mut choosen_x = vertices[0].x;
    let pick_max = choosen_x < 0.0;
    for i in 1..vertices.len() {
        let x = vertices[i].x;
        if pick_max {
            if x > choosen_x {
                choosen_index = i;
                choosen_x = x;
            }
        } else {
            if x < choosen_x {
                choosen_index = i;
                choosen_x = x;
            }
        }
    }
    choosen_index
}
