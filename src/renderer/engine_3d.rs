use crate::{
    draw_tools, nadk::display::{ScreenRect, push_rect, wait_for_vblank}, renderer::{
        clipping::{triangle_clip_against_line, triangle_clip_against_plane},
        draw_2d_triangles::clip_and_draw_2d_triangle,
        mesh::{IndexedTriangle2D, Mesh, MeshTriangle, Triangle, Triangle2D},
        *,
    }
};

impl Renderer {
    pub fn update_fov(&mut self, new_fov: f32) {
        self.camera.set_fov(new_fov);
        self.projection_matrix =
            Perspective3::new(ASPECT_RATIO, self.camera.get_fov(), ZNEAR, ZFAR);
    }

    pub fn clear_screen(&mut self, color: Color565) {
        self.tile_frame_buffer.fill(color);
    }

    fn add_3d_triangle_to_render(&mut self, mesh: &Mesh, tri_index: usize) {
        let tri = mesh.triangles[tri_index].clone();
        let camera_ray = mesh.vertices[tri.v1 as usize] - self.camera.get_pos();

        let a = mesh.vertices[tri.v2 as usize] - mesh.vertices[tri.v1 as usize];
        let b = mesh.vertices[tri.v3 as usize] - mesh.vertices[tri.v1 as usize];
        let tri_normal = a.cross(&b).normalize();

        //println!("{:?}",tri_normal);

        if tri_normal.dot(&camera_ray) < 0.0 {
            let clipped_triangles = triangle_clip_against_plane(
                &Vector3::new(0.0, 0.0, 0.1),
                &Vector3::new(0.0, 0.0, 1.0),
                &tri,
                &mut self.transformed_vertex_buffer
            );

            if let Some(clipped) = clipped_triangles.1.0 {
                self.project_single_vertex(clipped);
            }
            if let Some(clipped) = clipped_triangles.1.1 {
                self.project_single_vertex(clipped);
            }


            let mut project_and_add = |to_project: MeshTriangle| {
                let w1 = -self.transformed_vertex_buffer[to_project.v1 as usize].z;
                let w2 = -self.transformed_vertex_buffer[to_project.v2 as usize].z;
                let w3 = -self.transformed_vertex_buffer[to_project.v3 as usize].z;
                let t1 = Vector3::new((to_project.t1.x / w1) as f16, (to_project.t1.y / w1) as f16, (1.0 / w1) as f16);
                let t2 = Vector3::new((to_project.t2.x / w2) as f16, (to_project.t2.y / w2) as f16, (1.0 / w2) as f16);
                let t3 = Vector3::new((to_project.t3.x / w3) as f16, (to_project.t3.y / w3) as f16, (1.0 / w3) as f16);
                let projected_triangle = IndexedTriangle2D {
                    p1: self.projected_buffer[to_project.v1 as usize],
                    p2: self.projected_buffer[to_project.v2 as usize],
                    p3: self.projected_buffer[to_project.v3 as usize],
                    t1,
                    t2,
                    t3,
                };

                self.triangles_to_render.push(projected_triangle);
            };

            if let Some(clipped) = clipped_triangles.0.0 {
                project_and_add(clipped)
            }
            if let Some(clipped) = clipped_triangles.0.1 {
                project_and_add(clipped)
            }
        }
    }

    pub fn draw_triangles(&mut self, tile_x: usize, tile_y: usize) {
        let tile_offset = Vector2::new(
            -((SCREEN_TILE_WIDTH * tile_x) as i16),
            -((SCREEN_TILE_HEIGHT * tile_y) as i16),
        );
        for tri in self.triangles_to_render.iter_mut().rev() {
            let mut tri_copy = Triangle2D {p1: tri.p1, p2: tri.p2, p3: tri.p3, t1: tri.t1.map(|x| x as f32), t2: tri.t2.map(|x| x as f32), t3: tri.t3.map(|x| x as f32) };
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

    fn transform_verticies(
        &mut self,
        verticies: &[Vector3<f32>],
    ) {
        for vertex in verticies.iter() {
            let transformed: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = (self.mat_view * Vector4::new(vertex.x, vertex.y, vertex.z, 1.0)).xyz();
            self.transformed_vertex_buffer.push(transformed);
            
        }
    }

    fn project_verticies(&mut self)
    {
        for vertex in self.transformed_vertex_buffer.iter() {
            let projected = self.project_point(*vertex);
            let projected = (projected.xy() + Vector2::repeat(1.))
                        .component_mul(&HALF_SCREEN)
                        .map(|x| x as i16);
            self.projected_buffer.push(projected);
        }
    }

    fn project_single_vertex(&mut self, vertex: Vector3<f32>)
    {
        let projected = self.project_point(vertex);
        let projected = (projected.xy() + Vector2::repeat(1.))
                    .component_mul(&HALF_SCREEN)
                    .map(|x| x as i16);
        self.projected_buffer.push(projected);
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        self.clear_intermediate_buffers();
        self.transform_verticies(&mesh.vertices);
        self.project_verticies();
        for triangle in 0..mesh.triangles.len() {
            self.add_3d_triangle_to_render(mesh, triangle);
        }
    }
}
