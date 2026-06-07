use nalgebra::{Perspective3, Vector2, Vector3, Vector4};

use crate::{
    nadk::display::Color565,
    renderer::{ASPECT_RATIO, HALF_SCREEN, Renderer, ZFAR, ZNEAR},
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

    pub fn transform_verticies(&mut self, verticies: &[Vector3<f32>]) {
        for vertex in verticies.iter() {
            let transformed: nalgebra::Matrix<
                f32,
                nalgebra::Const<3>,
                nalgebra::Const<1>,
                nalgebra::ArrayStorage<f32, 3, 1>,
            > = (self.mat_view * Vector4::new(vertex.x, vertex.y, vertex.z, 1.0)).xyz();
            self.transformed_vertex_buffer.push(transformed);
        }
    }

    pub fn project_verticies(&mut self) {
        for vertex in self.transformed_vertex_buffer.iter() {
            let projected = self.project_point(*vertex);
            let projected = (projected.xy() + Vector2::repeat(1.))
                .component_mul(&HALF_SCREEN)
                .map(|x| x as i16);
            self.projected_buffer.push(projected);
        }
    }

    pub fn project_single_vertex(&mut self, vertex: Vector3<f32>) {
        let projected = self.project_point(vertex);
        let projected = (projected.xy() + Vector2::repeat(1.))
            .component_mul(&HALF_SCREEN)
            .map(|x| x as i16);
        self.projected_buffer.push(projected);
    }
}
