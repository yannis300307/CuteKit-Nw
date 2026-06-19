use nalgebra::{Vector2, Vector3};

use crate::renderer::{
    Renderer, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH,
    flat::{
        clipping::flat_triangle_clip_against_plane, draw_2d_triangle::clip_and_draw_2d_triangle,
    },
    mesh::{FlatCompactTriangle2D, FlatMesh, FlatMeshTriangle, FlatTriangle2D},
};

impl<'a> Renderer<'a> {
    fn add_3d_flat_triangle_to_render(&mut self, mesh: &FlatMesh, tri_index: usize) {
        let tri = mesh.triangles[tri_index].clone();
        let camera_ray = mesh.vertices[tri.v1 as usize] - self.camera.get_pos();

        let a = mesh.vertices[tri.v2 as usize] - mesh.vertices[tri.v1 as usize];
        let b = mesh.vertices[tri.v3 as usize] - mesh.vertices[tri.v1 as usize];
        let tri_normal = a.cross(&b).normalize();

        //println!("{:?}",tri_normal);

        if tri_normal.dot(&camera_ray) < 0.0 {
            let clipped_triangles = flat_triangle_clip_against_plane(
                &Vector3::new(0.0, 0.0, 0.1),
                &Vector3::new(0.0, 0.0, 1.0),
                &tri,
                &mut self.transformed_vertex_buffer,
            );

            if let Some(clipped) = clipped_triangles.1.0 {
                self.project_single_vertex(clipped);
            }
            if let Some(clipped) = clipped_triangles.1.1 {
                self.project_single_vertex(clipped);
            }

            let mut project_and_add = |to_project: FlatMeshTriangle| {
                let w1 = -self.transformed_vertex_buffer[to_project.v1 as usize].z;
                let w2 = -self.transformed_vertex_buffer[to_project.v2 as usize].z;
                let w3 = -self.transformed_vertex_buffer[to_project.v3 as usize].z;
                let projected_triangle = FlatCompactTriangle2D {
                    p1: self.projected_buffer[to_project.v1 as usize],
                    p2: self.projected_buffer[to_project.v2 as usize],
                    p3: self.projected_buffer[to_project.v3 as usize],
                    color: to_project.color,
                    depth: ((1.0 / w1) as f16, (1.0 / w2) as f16, (1.0 / w3) as f16),
                };

                self.flat_triangles_to_render.push(projected_triangle);
            };

            if let Some(clipped) = clipped_triangles.0.0 {
                project_and_add(clipped)
            }
            if let Some(clipped) = clipped_triangles.0.1 {
                project_and_add(clipped)
            }
        }
    }

    pub fn draw_flat_triangles(&mut self, tile_x: usize, tile_y: usize) {
        let tile_offset = Vector2::new(
            -((SCREEN_TILE_WIDTH * tile_x) as i16),
            -((SCREEN_TILE_HEIGHT * tile_y) as i16),
        );
        for tri in self.flat_triangles_to_render.iter_mut().rev() {
            let mut tri_copy = FlatTriangle2D {
                p1: tri.p1,
                p2: tri.p2,
                p3: tri.p3,
                depth: (tri.depth.0 as f32, tri.depth.1 as f32, tri.depth.2 as f32),
                color: tri.color,
            };
            tri_copy.p1 += tile_offset;

            tri_copy.p2 += tile_offset;

            tri_copy.p3 += tile_offset;

            clip_and_draw_2d_triangle(
                tri_copy,
                &mut self.tile_frame_buffer,
                &mut self.tile_depth_buffer,
            );
        }
    }

    pub fn draw_flat_mesh(&mut self, mesh: &FlatMesh) {
        self.clear_intermediate_buffers();
        self.transform_verticies(&mesh.vertices);
        self.project_verticies();
        for triangle in 0..mesh.triangles.len() {
            self.add_3d_flat_triangle_to_render(mesh, triangle);
        }
    }
}
