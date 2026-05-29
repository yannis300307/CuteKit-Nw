calc_use!(alloc::vec::Vec);

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
    pub p1: u16,
    pub p2: u16,
    pub p3: u16,
    pub t1: Vector3<f32>,
    pub t2: Vector3<f32>,
    pub t3: Vector3<f32>,
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

#[derive(Clone, Debug)]
pub struct MeshTriangle
{
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub t1: Vector2<f32>,
    pub t2: Vector2<f32>,
    pub t3: Vector2<f32>,
}

pub struct Mesh {
    pub triangles: Vec<MeshTriangle>,
    pub vertices: Vec<Vector3<f32>>,
    pub texture_coordinates: Vec<Vector2<f32>>
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            triangles: Vec::new(),
            vertices: Vec::new(),
            texture_coordinates: Vec::new()
        }
    }

    pub fn add_triangle(&mut self, triangle: MeshTriangle) {
        self.triangles.push(triangle);
    }

    pub fn add_vertex(&mut self, vertex: Vector3<f32>)
    {
        self.vertices.push(vertex);
    }
}
