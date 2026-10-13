use core::mem::swap;

use nalgebra::Vector2;

use crate::{
    nadk::display::Color565,
    renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH},
};

/*pub fn fill_triangle(
    mut p1: Vector2<isize>,
    mut p2: Vector2<isize>,
    mut p3: Vector2<isize>,
    depth: (f16, f16, f16),
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    color: Color565,
) {
    let depth = (depth.0 as f32, depth.1 as f32, depth.2 as f32);
    if p1.y > p2.y {
        swap(&mut p1, &mut p2);
    }
    if p1.y > p3.y {
        swap(&mut p1, &mut p3);
    }
    if p2.y > p3.y {
        swap(&mut p2, &mut p3);
    }

    let triangle_height = p3.y - p1.y;
    let triangle_heightf = triangle_height as f32;

    'height_iter: for i in 0..triangle_height {
        let second_half = i > (p2.y - p1.y) || (p2.y == p1.y);
        let segment_heightf = if second_half {
            (p3.y - p2.y) as f32
        } else {
            (p2.y - p1.y) as f32
        };

        let alpha = i as f32 / triangle_heightf;
        let beta = if second_half {
            (i as f32 - (p2.y - p1.y) as f32) / segment_heightf
        } else {
            i as f32 / segment_heightf
        };

        let mut a = p1.x as f32 + ((p3 - p1).x as f32 * alpha);
        let mut b = if second_half {
            p2.x as f32 + ((p3 - p2).x as f32 * beta)
        } else {
            p1.x as f32 + ((p2 - p1).x as f32 * beta)
        };

        if a > b {
            swap(&mut a, &mut b);
        }

        let y = p1.y + i;
        if y < 0 {
            continue 'height_iter;
        }
        if y >= SCREEN_TILE_HEIGHT as isize {
            break 'height_iter;
        }

        if (b as usize) < 1 {
            // prevent line bug
            continue;
        }

        for j in (a as usize)..=(b as usize) {
            if j >= SCREEN_TILE_WIDTH {
                continue 'height_iter;
            }
            frame_buffer[j + y as usize * SCREEN_TILE_WIDTH] = color;
        }
    }
}*/

#[inline(always)]
pub fn fill_triangle(
    frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
    mut point1: Vector2<i16>,
    mut depth1: f32,
    mut point2: Vector2<i16>,
    mut depth2: f32,
    mut point3: Vector2<i16>,
    mut depth3: f32,
    color: Color565,
) {
    if point2.y < point1.y {
        swap(&mut point1, &mut point2);
        swap(&mut depth1, &mut depth2);
    }

    if point3.y < point1.y {
        swap(&mut point1, &mut point3);
        swap(&mut depth1, &mut depth3);
    }

    if point3.y < point2.y {
        swap(&mut point2, &mut point3);
        swap(&mut depth2, &mut depth3);
    }

    let mut dpoint1 = point2 - point1;
    let mut ddepth1 = depth2 - depth1;

    let dpoint2 = point3 - point1;
    let ddepth2 = depth3 - depth1;

    let mut depth;

    let mut dax_step = 0.0;
    let mut dbx_step = 0.0;
    let mut ddepth1_step = 0.0;
    let mut ddepth2_step = 0.0;

    if dpoint1.y != 0 {
        dax_step = dpoint1.x as f32 / dpoint1.y.abs() as f32;
    }
    if dpoint2.y != 0 {
        dbx_step = dpoint2.x as f32 / dpoint2.y.abs() as f32;
    }

    if dpoint1.y != 0 {
        ddepth1_step = ddepth1 / (dpoint1.y.abs() as f32);
    }
    if dpoint2.y != 0 {
        ddepth2_step = ddepth2 / (dpoint2.y.abs() as f32);
    }

    if dpoint1.y != 0 {
        for i in point1.y..=point2.y {
            if i >= SCREEN_TILE_HEIGHT as i16 || i < 0 {
                continue;
            }
            let mut ax = (point1.x as f32 + (i - point1.y) as f32 * dax_step) as i16;
            let mut bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let mut depth_s = depth1 + (i - point1.y) as f32 * ddepth1_step;
            let mut depth_e = depth1 + (i - point1.y) as f32 * ddepth2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut depth_s, &mut depth_e);
            }

            let tstep = 1.0 / ((bx - ax) as f32);
            let mut t = 0.0;

            for j in ax..bx {
                if j >= SCREEN_TILE_WIDTH as i16 || j < 0 {
                    break;
                }
                depth = (1.0 - t) * depth_s + t * depth_e;
                let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;

                if depth < depth_buffer[index] as f32 {
                    unsafe { *frame_buffer.get_unchecked_mut(index) = color };
                    unsafe {
                        *depth_buffer.get_unchecked_mut(index) = depth as f16;
                    };
                }
                t += tstep;
            }
        }
    }

    dpoint1 = point3 - point2;
    ddepth1 = depth3 - depth2;

    if dpoint1.y != 0 {
        dax_step = dpoint1.x as f32 / dpoint1.y.abs() as f32;
    }
    if dpoint2.y != 0 {
        dbx_step = dpoint2.x as f32 / dpoint2.y.abs() as f32;
    }

    if dpoint1.y != 0 {
        ddepth1_step = ddepth1 / (dpoint1.y.abs() as f32);
    }

    if dpoint1.y != 0 {
        for i in point2.y..point3.y {
            if i >= SCREEN_TILE_HEIGHT as i16 || i < 0 {
                continue;
            }
            let mut ax = (point2.x as f32 + (i - point2.y) as f32 * dax_step) as i16;
            let mut bx = (point1.x as f32 + (i - point1.y) as f32 * dbx_step) as i16;

            let mut depth_s = depth2 + (i - point2.y) as f32 * ddepth1_step;
            let mut depth_e = depth1 + (i - point1.y) as f32 * ddepth2_step;

            if ax > bx {
                swap(&mut ax, &mut bx);
                swap(&mut depth_s, &mut depth_e);
            }

            let tstep = 1.0 / ((bx - ax) as f32);
            let mut t = 0.0;

            for j in ax..bx {
                if j >= SCREEN_TILE_WIDTH as i16 || j < 0 {
                    break;
                }
                depth = (1.0 - t) * depth_s + t * depth_e;
                let index = (i * SCREEN_TILE_WIDTH as i16 + j) as usize;

                if depth < depth_buffer[index] as f32 {
                    unsafe { *frame_buffer.get_unchecked_mut(index) = color };
                    unsafe {
                        *depth_buffer.get_unchecked_mut(index) = depth as f16;
                    };
                }
                t += tstep;
            }
        }
    }
}
