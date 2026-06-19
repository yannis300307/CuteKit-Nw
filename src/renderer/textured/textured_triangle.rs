use core::mem::swap;

use nalgebra::{Vector2, Vector3};

use crate::{
    nadk::display::Color565,
    renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH}, renderer2d::elements::Texture,
};

// #[inline(always)]
fn scan_line(
    mut ax: i16,
    mut bx: i16,
    mut tex_s: Vector3<f32>,
    mut tex_e: Vector3<f32>,
    i: i16,
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    texture: &Texture
) {
    if ax > bx {
        swap(&mut ax, &mut bx);
        swap(&mut tex_s, &mut tex_e);
    }

    let tstep: f32 = 1.0 / ((bx - ax) as f32);
    let mut t = 0.0;

    let texture_widthf = texture.width as f32;
    let texture_heightf = texture.height as f32;

    for j in ax..bx {
        let tex_coords = (1.0 - t) * tex_s + t * tex_e;
        let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;
        let z_inv = 1.0 / tex_coords.z;

        let u = (tex_coords.x * z_inv).clamp(0.0, 0.9999);
        let v = (tex_coords.y * z_inv).clamp(0.0, 0.9999);


        if tex_coords.z < depth_buffer[index] as f32 {
            let texture_pixel_index = ((u * texture_widthf) as usize) + ((v * texture_heightf) as usize) * texture.width as usize;
            let pixel: Color565 = texture.data[texture_pixel_index];
            frame_buffer[index] = pixel;    
            depth_buffer[index] = tex_coords.z as f16;
        }
        t += tstep;
    }
}

//#[inline(always)]
pub fn textured_triangle(
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    mut point1: Vector2<i16>,
    mut tex1: Vector3<f32>,
    mut point2: Vector2<i16>,
    mut tex2: Vector3<f32>,
    mut point3: Vector2<i16>,
    mut tex3: Vector3<f32>,
    texture: &Texture
) {
    if point2.y < point1.y {
        swap(&mut point1, &mut point2);
        swap(&mut tex1, &mut tex2);
    }

    if point3.y < point1.y {
        swap(&mut point1, &mut point3);
        swap(&mut tex1, &mut tex3);
    }

    if point3.y < point2.y {
        swap(&mut point2, &mut point3);
        swap(&mut tex2, &mut tex3);
    }

    let mut dpoint1 = point2 - point1;
    let mut dtex1 = tex2 - tex1;

    let dpoint2 = point3 - point1;
    let dtex2 = tex3 - tex1;

    let mut dax_step = 0.0;
    let mut dbx_step = 0.0;
    let mut dtex1_step = Vector3::repeat(0.0);
    let mut dtex2_step = Vector3::repeat(0.0);

    if dpoint1.y != 0 {
        dax_step = dpoint1.x as f32 / dpoint1.y.abs() as f32;
    }
    if dpoint2.y != 0 {
        dbx_step = dpoint2.x as f32 / dpoint2.y.abs() as f32;
    }

    if dpoint1.y != 0 {
        dtex1_step = dtex1 / (dpoint1.y.abs() as f32);
    }
    if dpoint2.y != 0 {
        dtex2_step = dtex2 / (dpoint2.y.abs() as f32);
    }

    if dpoint1.y != 0 {
        for i in point1.y..=point2.y {
            if i >= SCREEN_TILE_HEIGHT as i16 || i < 0 {
                continue;
            }
            let ax = (point1.x as f32 + (i - point1.y) as f32 * dax_step) as i16;
            let bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let tex_s = tex1 + (i - point1.y) as f32 * dtex1_step;
            let tex_e = tex1 + (i - point1.y) as f32 * dtex2_step;

            scan_line(ax, bx, tex_s, tex_e, i, frame_buffer, depth_buffer, texture);
        }
    }

    dpoint1 = point3 - point2;
    dtex1 = tex3 - tex2;

    if dpoint1.y != 0 {
        dax_step = dpoint1.x as f32 / dpoint1.y.abs() as f32;
    }
    if dpoint2.y != 0 {
        dbx_step = dpoint2.x as f32 / dpoint2.y.abs() as f32;
    }

    dtex1_step.x = 0.0;
    dtex1_step.y = 0.0;
    if dpoint1.y != 0 {
        dtex1_step = dtex1 / (dpoint1.y.abs() as f32);
    }

    if dpoint1.y != 0 {
        for i in point2.y..point3.y {
            let ax = (point2.x as f32 + (i - point2.y) as f32 * dax_step) as i16;
            let bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let tex_s = tex2 + (i - point2.y) as f32 * dtex1_step;
            let tex_e = tex1 + (i - point1.y) as f32 * dtex2_step;

            scan_line(ax, bx, tex_s, tex_e, i, frame_buffer, depth_buffer, texture);
        }
    }
}
