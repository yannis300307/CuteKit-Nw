use nalgebra::{Vector2, Vector3};

use crate::{nadk::display::Color565, renderer::{
    Renderer, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH,
    mesh::{TexCompactTriangle2D, TexMeshTriangle, TexTriangle2D, TexturedMesh},
    textured::{
        clipping::tex_triangle_clip_against_plane, draw_2d_triangles::clip_and_draw_2d_triangle,
    },
}, renderer2d::elements::Texture};

impl<'a> Renderer<'a> {
    fn add_3d_textured_triangle_to_render(&mut self, mesh: &TexturedMesh, tri_index: usize) {
        let tri = mesh.triangles[tri_index].clone();
        let camera_ray = mesh.vertices[tri.v1 as usize] - self.camera.get_pos();

        let a = mesh.vertices[tri.v2 as usize] - mesh.vertices[tri.v1 as usize];
        let b = mesh.vertices[tri.v3 as usize] - mesh.vertices[tri.v1 as usize];
        let tri_normal = a.cross(&b).normalize();

        //println!("{:?}",tri_normal);

        if tri_normal.dot(&camera_ray) < 0.0 {
            let clipped_triangles = tex_triangle_clip_against_plane(
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

            let mut project_and_add = |to_project: TexMeshTriangle| {
                let w1 = -self.transformed_vertex_buffer[to_project.v1 as usize].z;
                let w2 = -self.transformed_vertex_buffer[to_project.v2 as usize].z;
                let w3 = -self.transformed_vertex_buffer[to_project.v3 as usize].z;
                let t1 = Vector3::new(
                    (to_project.t1.x / w1) as f16,
                    (to_project.t1.y / w1) as f16,
                    (1.0 / w1) as f16,
                );
                let t2 = Vector3::new(
                    (to_project.t2.x / w2) as f16,
                    (to_project.t2.y / w2) as f16,
                    (1.0 / w2) as f16,
                );
                let t3 = Vector3::new(
                    (to_project.t3.x / w3) as f16,
                    (to_project.t3.y / w3) as f16,
                    (1.0 / w3) as f16,
                );
                let projected_triangle = TexCompactTriangle2D {
                    p1: self.projected_buffer[to_project.v1 as usize],
                    p2: self.projected_buffer[to_project.v2 as usize],
                    p3: self.projected_buffer[to_project.v3 as usize],
                    t1,
                    t2,
                    t3,
                };

                self.tex_triangles_to_render.push(projected_triangle);
            };

            if let Some(clipped) = clipped_triangles.0.0 {
                project_and_add(clipped)
            }
            if let Some(clipped) = clipped_triangles.0.1 {
                project_and_add(clipped)
            }
        }
    }

    pub fn draw_tex_triangles(&mut self, offset: Vector2<isize>, texture: &Texture, frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT]) {
        let offset = offset.map(|x | x as i16);
        for tri in self.tex_triangles_to_render.iter_mut().rev() {
            let mut tri_copy = TexTriangle2D {
                p1: tri.p1,
                p2: tri.p2,
                p3: tri.p3,
                t1: tri.t1.map(|x| x as f32),
                t2: tri.t2.map(|x| x as f32),
                t3: tri.t3.map(|x| x as f32),
            };
            tri_copy.p1 -= offset;

            tri_copy.p2 -= offset;

            tri_copy.p3 -= offset;

            clip_and_draw_2d_triangle(
                tri_copy,
                frame_buffer,
                &mut self.tile_depth_buffer,
                texture
            );
        }
    }

    pub fn draw_textured_mesh(&mut self, mesh: &TexturedMesh) {
        self.clear_intermediate_buffers();
        self.transform_verticies(&mesh.vertices);
        self.project_verticies();
        for triangle in 0..mesh.triangles.len() {
            self.add_3d_textured_triangle_to_render(mesh, triangle);
        }
    }
}
