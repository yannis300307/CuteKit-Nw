use nalgebra::Vector2;

use crate::{nadk::display::Color565, renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, flat::{clipping::flat_triangle_clip_against_line, flat_triangle::fill_triangle}, mesh::FlatTriangle2D}};

pub fn draw_2d_flat_triangles(
    tri: &FlatTriangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    // Normal Triangle
    fill_triangle(
                frame_buffer,
        depth_buffer,
        tri.p1,
        tri.depth.0,
        tri.p2,
        tri.depth.1,
        tri.p3,
        tri.depth.2,
        tri.color
    );
}


pub fn clip_and_draw_2d_triangle(
    tri: FlatTriangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    let mut clip_buffer: heapless::Deque<FlatTriangle2D, 16> = heapless::Deque::new(); // 2^4

    clip_buffer.push_back(tri).unwrap();
    let mut new_tris = 1;

    let mut clip_triangle = |line_p, line_n| {
        while new_tris > 0 {
            let test = clip_buffer.pop_front().unwrap();
            new_tris -= 1;

            let clipped = flat_triangle_clip_against_line(&line_p, &line_n, &test);

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
        draw_2d_flat_triangles(&cliped_tri, frame_buffer, depth_buffer);
    }
}
