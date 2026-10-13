use core::mem::{swap, transmute};

use nalgebra::{Vector2, Vector3};

use crate::{
    nadk::display::Color565,
    renderer2d::{
        elements::Texture,
        renderer::{Renderer2d, SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH},
        sprite::{TransparentRGB565, TransparentTexture, add_alpha_color},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct TexTriangle2D {
    pub p1: Vector2<i16>,
    pub p2: Vector2<i16>,
    pub p3: Vector2<i16>,
    pub t1: Vector2<f32>,
    pub t2: Vector2<f32>,
    pub t3: Vector2<f32>,
}

pub fn vector_intersect_line(
    line_p: &Vector2<f32>,
    line_n: &Vector2<f32>,
    point_start: &Vector2<f32>,
    point_end: &Vector2<f32>,
) -> (Vector2<i16>, f32) {
    let line_n = line_n.normalize();
    let line_d = -line_n.dot(line_p);
    let ad = point_start.dot(&line_n);
    let bd = point_end.dot(&line_n);
    let t = (-line_d - ad) / (bd - ad);
    let point_start_to_end = point_end - point_start;
    let point_to_intersect = point_start_to_end * t;
    let coords = point_start + point_to_intersect;
    (coords.map(|x| x as i16), t)
}

pub fn triangle_clip_against_line(
    line_p: &Vector2<f32>,
    line_n: &Vector2<f32>,
    in_tri: &TexTriangle2D,
) -> (Option<TexTriangle2D>, Option<TexTriangle2D>) {
    let line_n = line_n.normalize();

    let dist = |p: Vector2<f32>| line_n.x * p.x + line_n.y * p.y - line_n.dot(line_p);

    let default = Default::default();
    let mut inside_points: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_inside_point_count = 0;
    let mut outside_points: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_outside_point_count = 0;

    let default = Default::default();
    let mut inside_tex: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_inside_tex_count = 0;
    let mut outside_tex: [&Vector2<f32>; 3] = [&default; 3];
    let mut n_outside_tex_count = 0;

    // TODO: Why is the i16 cast into a f32 ????!!! I wrote that but I don't remember why I did that...
    let p1 = Vector2::new(in_tri.p1.x as f32, in_tri.p1.y as f32);
    let p2 = Vector2::new(in_tri.p2.x as f32, in_tri.p2.y as f32);
    let p3 = Vector2::new(in_tri.p3.x as f32, in_tri.p3.y as f32);

    let d0 = dist(p1);
    let d1 = dist(p2);
    let d2 = dist(p3);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count] = &p1;
        inside_tex[n_inside_tex_count] = &in_tri.t1;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p1;
        outside_tex[n_outside_tex_count] = &in_tri.t1;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count] = &p2;
        inside_tex[n_inside_tex_count] = &in_tri.t2;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p2;
        outside_tex[n_outside_tex_count] = &in_tri.t2;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count] = &p3;
        inside_tex[n_inside_tex_count] = &in_tri.t3;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = &p3;
        outside_tex[n_outside_tex_count] = &in_tri.t3;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }

    if n_inside_point_count == 0 {
        return (None, None);
    }

    if n_inside_point_count == 3 {
        return (Some(*in_tri), None);
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        let p1 = inside_points[0];
        let (p2, t) = vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]);
        let t2 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let (p3, t) = vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[1]);
        let t3 = t * (outside_tex[1] - inside_tex[0]) + inside_tex[0];
        let out_tri = TexTriangle2D {
            p1: p1.map(|x| x as i16),
            p2,
            p3,
            t1: *inside_tex[0],
            t2,
            t3,
        };

        return (Some(out_tri), None);
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let p1 = inside_points[0];
        let p2 = inside_points[1];
        let (p3, t) = vector_intersect_line(line_p, &line_n, inside_points[0], outside_points[0]);
        let t3 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let out_tri1 = TexTriangle2D {
            p1: p1.map(|x| x as i16),
            p2: p2.map(|x| x as i16),
            p3,
            t1: *inside_tex[0],
            t2: *inside_tex[1],
            t3,
        };

        let (p3, t) = vector_intersect_line(line_p, &line_n, inside_points[1], outside_points[0]);
        let t3 = t * (outside_tex[0] - inside_tex[1]) + inside_tex[1];
        let out_tri2 = TexTriangle2D {
            p1: inside_points[1].map(|x: f32| x as i16),
            p2: out_tri1.p3,
            p3,
            t1: *inside_tex[1],
            t2: out_tri1.t3,
            t3,
        };
        return (Some(out_tri1), Some(out_tri2));
    }
    (None, None)
}

impl Renderer2d {
    // #[inline(always)]
    fn scan_line(
        &mut self,
        mut ax: i16,
        mut bx: i16,
        mut tex_s: Vector2<f32>,
        mut tex_e: Vector2<f32>,
        i: i16,
        texture: &TransparentTexture,
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

            let u = (tex_coords.x).clamp(0.0, 0.9999);
            let v = (tex_coords.y).clamp(0.0, 0.9999);

            let texture_pixel_index = ((u * texture_widthf) as usize)
                + ((v * texture_heightf) as usize) * texture.width as usize;
            let pixel = texture.data[texture_pixel_index];

            let old_color = self.tile_frame_buffer[index];
            
            self.tile_frame_buffer[index] = add_alpha_color(old_color, pixel);
            t += tstep;
        }
    }

    pub fn textured_triangle(
        &mut self,
        mut point1: Vector2<i16>,
        mut tex1: Vector2<f32>,
        mut point2: Vector2<i16>,
        mut tex2: Vector2<f32>,
        mut point3: Vector2<i16>,
        mut tex3: Vector2<f32>,
        texture: &TransparentTexture,
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
        let mut dtex1_step = Vector2::repeat(0.0);
        let mut dtex2_step = Vector2::repeat(0.0);

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

                self.scan_line(ax, bx, tex_s, tex_e, i, texture);
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

                self.scan_line(ax, bx, tex_s, tex_e, i, texture);
            }
        }
    }

    fn draw_2d_triangles(&mut self, tri: &TexTriangle2D, texture: &TransparentTexture) {
        // Normal Triangle
        self.textured_triangle(tri.p1, tri.t1, tri.p2, tri.t2, tri.p3, tri.t3, texture);
    }

    pub(super) fn clip_and_draw_2d_triangle(
        &mut self,
        tri: TexTriangle2D,
        texture: &TransparentTexture,
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
            self.draw_2d_triangles(&cliped_tri, texture);
        }
    }
}
