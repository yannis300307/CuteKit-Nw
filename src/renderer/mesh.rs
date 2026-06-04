calc_use!(alloc::vec::Vec);

use bytemuck::{Pod, Zeroable};
use nalgebra::{Vector2, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
    pub p3: Vector3<f32>,
    pub t1: Vector2<f32>,
    pub t2: Vector2<f32>,
    pub t3: Vector2<f32>,
}

#[derive(Clone, Copy, Debug)]
pub struct IndexedTriangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub t1: Vector3<f16>,
    pub t2: Vector3<f16>,
    pub t3: Vector3<f16>,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub t1: Vector3<f32>,
    pub t2: Vector3<f32>,
    pub t3: Vector3<f32>,
}

impl Triangle {
    pub fn get_normal(&self) -> Vector3<f32> {
        let a = self.p2 - self.p1;
        let b = self.p3 - self.p1;
        a.cross(&b).normalize()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct MeshTriangle
{
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub(crate) padding: u16,
    pub t1: Vector2<f32>,
    pub t2: Vector2<f32>,
    pub t3: Vector2<f32>,
}

pub struct Mesh {
    pub triangles: &'static [MeshTriangle],
    pub vertices: &'static [Vector3<f32>],
}
