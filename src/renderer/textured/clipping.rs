calc_use!(alloc::vec::Vec);
use nalgebra::{Vector2, Vector3};

use crate::renderer::{
    clipping::{vector_intersect_line, vector_intersect_plane},
    mesh::{TexMeshTriangle, TexTriangle2D},
};

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
    let mut inside_tex: [&Vector3<f32>; 3] = [&default; 3];
    let mut n_inside_tex_count = 0;
    let mut outside_tex: [&Vector3<f32>; 3] = [&default; 3];
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

/* Clips the triangle against the described lines.
Can return additional vertex created during the clipping that must be projected.*/
pub fn tex_triangle_clip_against_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    in_tri: &TexMeshTriangle,
    verticies: &mut Vec<Vector3<f32>>,
) -> (
    (Option<TexMeshTriangle>, Option<TexMeshTriangle>),
    (Option<Vector3<f32>>, Option<Vector3<f32>>),
) {
    let plane_n = plane_n.normalize();

    let dist = |p: Vector3<f32>| {
        plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - plane_n.dot(plane_p)
    };

    let mut inside_points: [u16; 3] = [0; 3];
    let mut n_inside_point_count = 0;
    let mut outside_points: [u16; 3] = [0; 3];
    let mut n_outside_point_count = 0;

    let temp = Vector2::zeros();
    let mut inside_tex: [&Vector2<f32>; 3] = [&temp; 3];
    let mut n_inside_tex_count = 0;
    let mut outside_tex: [&Vector2<f32>; 3] = [&temp; 3];
    let mut n_outside_tex_count = 0;

    let d0 = dist(verticies[in_tri.v1 as usize]);
    let d1 = dist(verticies[in_tri.v2 as usize]);
    let d2 = dist(verticies[in_tri.v3 as usize]);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count] = in_tri.v1;
        inside_tex[n_inside_tex_count] = &in_tri.t1;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = in_tri.v1;
        outside_tex[n_outside_tex_count] = &in_tri.t1;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count] = in_tri.v2;
        inside_tex[n_inside_tex_count] = &in_tri.t2;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = in_tri.v2;
        outside_tex[n_outside_tex_count] = &in_tri.t2;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count] = in_tri.v3;
        inside_tex[n_inside_tex_count] = &in_tri.t3;
        n_inside_tex_count += 1;
        n_inside_point_count += 1;
    } else {
        outside_points[n_outside_point_count] = in_tri.v3;
        outside_tex[n_outside_tex_count] = &in_tri.t3;
        n_outside_tex_count += 1;
        n_outside_point_count += 1;
    }

    if n_inside_point_count == 0 {
        return ((None, None), (None, None));
    }

    if n_inside_point_count == 3 {
        return ((Some(in_tri.clone()), None), (None, None));
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        // Well yeah, this can be a bit weird but the extra_vX variables are used later to add the missing projected verticies to the projected buffer.
        let (extra_v2, t) = vector_intersect_plane(
            plane_p,
            &plane_n,
            &verticies[inside_points[0] as usize],
            &verticies[outside_points[0] as usize],
        );
        verticies.push(extra_v2);
        let v2 = (verticies.len() - 1) as u16;
        let t2 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let (extra_v3, t) = vector_intersect_plane(
            plane_p,
            &plane_n,
            &verticies[inside_points[0] as usize],
            &verticies[outside_points[1] as usize],
        );
        verticies.push(extra_v3);
        let t3 = t * (outside_tex[1] - inside_tex[0]) + inside_tex[0];
        let out_tri = TexMeshTriangle {
            v1: inside_points[0],
            v2,
            v3: v2 + 1,
            padding: 0,
            t1: *inside_tex[0],
            t2,
            t3,
        };

        return ((Some(out_tri), None), (Some(extra_v2), Some(extra_v3)));
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        let (extra_v3_1, t) = vector_intersect_plane(
            plane_p,
            &plane_n,
            &verticies[inside_points[0] as usize],
            &verticies[outside_points[0] as usize],
        );
        verticies.push(extra_v3_1);
        let v3 = (verticies.len() - 1) as u16;
        let t3 = t * (outside_tex[0] - inside_tex[0]) + inside_tex[0];
        let out_tri1 = TexMeshTriangle {
            v1: inside_points[0],
            v2: inside_points[1],
            v3,
            padding: 0,
            t1: *inside_tex[0],
            t2: *inside_tex[1],
            t3,
        };

        let (extra_v3_2, t) = vector_intersect_plane(
            plane_p,
            &plane_n,
            &verticies[inside_points[1] as usize],
            &verticies[outside_points[0] as usize],
        );
        verticies.push(extra_v3_2);
        let v3 = (verticies.len() - 1) as u16;
        let t3 = t * (outside_tex[0] - inside_tex[1]) + inside_tex[1];
        let out_tri2 = TexMeshTriangle {
            v1: inside_points[1],
            v2: out_tri1.v3,
            v3,
            padding: 0,
            t1: *inside_tex[1],
            t2: out_tri1.t3,
            t3,
        };
        return (
            (Some(out_tri1), Some(out_tri2)),
            (Some(extra_v3_1), Some(extra_v3_2)),
        );
    }
    ((None, None), (None, None))
}
