use bytemuck::{Pod, Zeroable};
use nalgebra::{Vector2, Vector3};

use crate::nadk::display::Color565;

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct FlatMeshTriangle {
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub color: Color565,
}

#[derive(Clone, Copy, Debug)]
pub struct FlatCompactTriangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub color: Color565,
    pub depth: (f16, f16, f16)
}

#[derive(Clone, Copy, Debug)]
pub struct TexCompactTriangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub t1: Vector3<f16>,
    pub t2: Vector3<f16>,
    pub t3: Vector3<f16>,
}

#[derive(Clone, Copy, Debug)]
pub struct TexTriangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub t1: Vector3<f32>,
    pub t2: Vector3<f32>,
    pub t3: Vector3<f32>,
}

#[derive(Clone, Copy, Debug)]
pub struct FlatTriangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub depth: (f32, f32, f32),
    pub color: Color565
}


#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct TexMeshTriangle {
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub(crate) padding: u16,
    pub t1: Vector2<f32>,
    pub t2: Vector2<f32>,
    pub t3: Vector2<f32>,
}

pub struct TexturedMesh {
    pub triangles: &'static [TexMeshTriangle],
    pub vertices: &'static [Vector3<f32>],
}

pub struct FlatMesh {
    pub triangles: &'static [FlatMeshTriangle],
    pub vertices: &'static [Vector3<f32>],
}
