use nalgebra::Vector2;

use crate::{
    nadk::display::Color565,
    renderer::{
        SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH,
        mesh::TexTriangle2D,
        textured::{clipping::triangle_clip_against_line, textured_triangle::textured_triangle},
    }, renderer2d::elements::Texture,
};

pub fn draw_2d_triangles(
    tri: &TexTriangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    texture:&Texture 
) {
    // Normal Triangle
    textured_triangle(
        frame_buffer,
        depth_buffer,
        tri.p1,
        tri.t1,
        tri.p2,
        tri.t2,
        tri.p3,
        tri.t3,
        texture
    );
}

// Takes a Triangle2D and draw it as a filled triangle or lines depending of the texture_id
pub fn clip_and_draw_2d_triangle(
    tri: TexTriangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    texture: &Texture
) {
    let mut clip_buffer: heapless::Deque<TexTriangle2D, 16> = heapless::Deque::new(); // 2^4

    clip_buffer.push_back(tri).unwrap();
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
    clip_triangle(
        Vector2::new(0.0, SCREEN_TILE_HEIGHT as f32),
        Vector2::new(0.0, -1.0),
    );
    clip_triangle(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0));
    clip_triangle(
        Vector2::new(SCREEN_TILE_WIDTH as f32, 0.0),
        Vector2::new(-1.0, 0.0),
    );

    for cliped_tri in clip_buffer {
        draw_2d_triangles(&cliped_tri, frame_buffer, depth_buffer, texture);
    }
}
