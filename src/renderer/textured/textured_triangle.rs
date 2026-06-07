use core::mem::swap;

use nalgebra::{Vector2, Vector3};

use crate::{
    nadk::display::Color565,
    renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH, TEXTURE},
};

#[inline(always)]
pub fn textured_triangle(
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    mut point1: Vector2<i16>,
    mut tex1: Vector3<f32>,
    mut point2: Vector2<i16>,
    mut tex2: Vector3<f32>,
    mut point3: Vector2<i16>,
    mut tex3: Vector3<f32>,
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

    let mut tex_coords;

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
            let mut ax = (point1.x as f32 + (i - point1.y) as f32 * dax_step) as i16;
            let mut bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let mut tex_s = tex1 + (i - point1.y) as f32 * dtex1_step;
            let mut tex_e = tex1 + (i - point1.y) as f32 * dtex2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut tex_s, &mut tex_e);
            }

            let tstep: f32 = 1.0 / ((bx - ax) as f32);
            let mut t = 0.0;

            for j in ax..bx {
                if j >= SCREEN_TILE_WIDTH as i16 || j < 0 {
                    break;
                }
                tex_coords = (1.0 - t) * tex_s + t * tex_e;
                let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;
                let z_inv = 1.0 / tex_coords.z;

                let u = (tex_coords.x * z_inv).clamp(0.0, 0.9999);
                let v = (tex_coords.y * z_inv).clamp(0.0, 0.9999);

                if tex_coords.z < depth_buffer[index] as f32 {
                    let texture_pixel_index =
                        (((u * 512.0) as usize) + ((v * 512.0) as usize) * 512) * 2;
                    let pixel = u16::from_be_bytes([
                        *unsafe { TEXTURE.get_unchecked(texture_pixel_index) },
                        *unsafe { TEXTURE.get_unchecked(texture_pixel_index + 1) },
                    ]);
                    unsafe { *frame_buffer.get_unchecked_mut(index) = Color565 { value: pixel } };
                    unsafe {
                        *depth_buffer.get_unchecked_mut(index) = tex_coords.z as f16;
                    };
                }
                t += tstep;
            }
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
            if i >= SCREEN_TILE_HEIGHT as i16 || i < 0 {
                continue;
            }
            let mut ax = (point2.x as f32 + (i - point2.y) as f32 * dax_step) as i16;
            let mut bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let mut tex_s = tex2 + (i - point2.y) as f32 * dtex1_step;
            let mut tex_e = tex1 + (i - point1.y) as f32 * dtex2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut tex_s, &mut tex_e);
            }

            let tstep = 1.0 / ((bx - ax) as f32);
            let mut t = 0.0;

            for j in ax..bx {
                if j >= SCREEN_TILE_WIDTH as i16 || j < 0 {
                    break;
                }
                tex_coords = (1.0 - t) * tex_s + t * tex_e;
                let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;
                let z_inv = 1.0 / tex_coords.z;

                let u = (tex_coords.x * z_inv).clamp(0.0, 0.9999);
                let v = (tex_coords.y * z_inv).clamp(0.0, 0.9999);

                if tex_coords.z < depth_buffer[index] as f32 {
                    let texture_pixel_index =
                        (((u * 512.0) as usize) + ((v * 512.0) as usize) * 512) * 2;
                    let pixel = u16::from_be_bytes([
                        *unsafe { TEXTURE.get_unchecked(texture_pixel_index) },
                        *unsafe { TEXTURE.get_unchecked(texture_pixel_index + 1) },
                    ]);
                    unsafe { *frame_buffer.get_unchecked_mut(index) = Color565 { value: pixel } };
                    unsafe {
                        *depth_buffer.get_unchecked_mut(index) = tex_coords.z as f16;
                    };
                }
                t += tstep;
            }
        }
    }
}
