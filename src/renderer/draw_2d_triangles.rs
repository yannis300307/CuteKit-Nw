use nalgebra::Vector2;

use crate::{
    nadk::display::Color565,
    renderer::{
        SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, clipping::triangle_clip_against_line,
        mesh::{IndexedTriangle2D, Triangle2D}, misc::draw_line, textured_triangle::textured_triangle,
    },
};

pub fn draw_2d_triangles(
    tri: &Triangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
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
        //get_quad_color_from_texture_id(tri.texture_id).apply_light(tri.light * 17),
    );
    /*draw_line(
        (tri.p1.x as isize, tri.p1.y as isize),
        (tri.p2.x as isize, tri.p2.y as isize),
        frame_buffer,
        Color565::new(0b11111, 0b0, 0b0),
    );
    draw_line(
        (tri.p2.x as isize, tri.p2.y as isize),
        (tri.p3.x as isize, tri.p3.y as isize),
        frame_buffer,
        Color565::new(0b11111, 0b0, 0b0),
    );
    draw_line(
        (tri.p3.x as isize, tri.p3.y as isize),
        (tri.p1.x as isize, tri.p1.y as isize),
        frame_buffer,
        Color565::new(0b11111, 0b0, 0b0),
    );*/
}

// Takes a Triangle2D and draw it as a filled triangle or lines depending of the texture_id
pub fn clip_and_draw_2d_triangle(
    tri: Triangle2D,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
) {
    let mut clip_buffer: heapless::Deque<Triangle2D, 16> = heapless::Deque::new(); // 2^4

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
        draw_2d_triangles(&cliped_tri, frame_buffer, depth_buffer);
    }
}
