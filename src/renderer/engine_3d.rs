use crate::{
    nadk::display::{ScreenRect, push_rect, wait_for_vblank},
    renderer::{
        clipping::{triangle_clip_against_line, triangle_clip_against_plane},
        draw_2d_triangles::clip_and_draw_2d_triangle,
        mesh::{IndexedTriangle2D, Mesh, MeshTriangle, Triangle, Triangle2D},
        *,
    },
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

            let mut project_and_add = |to_project: MeshTriangle| {
                let w1 = -self.transformed_vertex_buffer[to_project.v1 as usize].z;
                let w2 = -self.transformed_vertex_buffer[to_project.v2 as usize].z;
                let w3 = -self.transformed_vertex_buffer[to_project.v3 as usize].z;
                let t1 = Vector3::new(to_project.t1.x / w1, to_project.t1.y / w1, 1.0 / w1);
                let t2 = Vector3::new(to_project.t2.x / w2, to_project.t2.y / w2, 1.0 / w2);
                let t3 = Vector3::new(to_project.t3.x / w3, to_project.t3.y / w3, 1.0 / w3);
                let projected_triangle = IndexedTriangle2D {
                    p1: to_project.v1,
                    p2: to_project.v2,
                    p3: to_project.v3,
                    t1,
                    t2,
                    t3,
                };

                /*let mut clip_buffer: heapless::Deque<Triangle2D, 16> = heapless::Deque::new(); // 2^4

                clip_buffer.push_back(projected_triangle).unwrap();
                let mut new_tris = 1;

                let mut clip_triangle = |line_p, line_n| {
                    while new_tris > 0 {
                        let test = clip_buffer.pop_front().unwrap();
                        new_tris -= 1;

                        let clipped = triangle_clip_against_line(&line_p, &line_n, &test);

                        if let Some(clipped_tri) = clipped.0 {
                            clip_buffer.push_back(clipped_tri).unwrap();
                        }
                        if let Some(clipped_tri) = clipped.1 {
                            clip_buffer.push_back(clipped_tri).unwrap();
                        }
                    }
                    new_tris = clip_buffer.len();
                };

                clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0));
                clip_triangle(Vector2::new(0.0, SCREEN_HEIGHTF), Vector2::new(0.0, -1.0));
                clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
                clip_triangle(Vector2::new(SCREEN_WIDTHF, 0.0), Vector2::new(-1.0, 0.0));

                for tri in clip_buffer {
                    if self.triangles_to_render.len() >= MAX_TRIANGLES {
                        // TODO : Find a proper fix for this
                        break;
                    }
                    self.triangles_to_render.push(tri.to_small()); // Do nothing if overflow
                }*/

                self.triangles_to_render.push(projected_triangle);
            };

            if let Some(clipped) = clipped_triangles.0 {
                project_and_add(clipped)
            }
            if let Some(clipped) = clipped_triangles.1 {
                project_and_add(clipped)
            }
        }
    }

    fn draw_triangles(&mut self, tile_x: usize, tile_y: usize) {
        let tile_offset = Vector2::new(
            -((SCREEN_TILE_WIDTH * tile_x) as i16),
            -((SCREEN_TILE_HEIGHT * tile_y) as i16),
        );
        for tri in self.triangles_to_render.iter_mut().rev() {
            let mut tri_copy = Triangle2D {p1: self.projected_buffer[tri.p1 as usize], p2: self.projected_buffer[tri.p2 as usize], p3: self.projected_buffer[tri.p3 as usize], t1: tri.t1, t2: tri.t2, t3: tri.t3 };
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
        verticies: &Vec<Vector3<f32>>,
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

    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        self.transform_verticies(&mesh.vertices);
        for triangle in 0..mesh.triangles.len() {
            self.add_3d_triangle_to_render(mesh, triangle);
        }
        self.project_verticies();
    }

    pub fn draw_game(&mut self) {
        self.mat_view = self.get_mat_view();

        for x in 0..SCREEN_TILE_SUBDIVISION {
            for y in 0..SCREEN_TILE_SUBDIVISION {
                self.clear_screen(Color565::new(0b01110, 0b110110, 0b11111));
                for i in 0..SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT {
                    self.tile_depth_buffer[i] = f16::MAX;
                }
                self.draw_triangles(x, y);

                push_rect(
                    ScreenRect {
                        x: (SCREEN_TILE_WIDTH * x) as u16,
                        y: (SCREEN_TILE_HEIGHT * y) as u16,
                        width: SCREEN_TILE_WIDTH as u16,
                        height: SCREEN_TILE_HEIGHT as u16,
                    },
                    &self.tile_frame_buffer,
                );
            }
        }
        if self.enable_vsync {
            wait_for_vblank();
        }
        self.triangles_to_render.clear();
        self.transformed_vertex_buffer.clear();
        self.projected_buffer.clear();
    }
}
