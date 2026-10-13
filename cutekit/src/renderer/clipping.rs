use nalgebra::{Vector2, Vector3};

pub fn vector_intersect_plane(
    plane_p: &Vector3<f32>,
    plane_n: &Vector3<f32>,
    line_start: &Vector3<f32>,
    line_end: &Vector3<f32>,
) -> (Vector3<f32>, f32) {
    let plane_n = plane_n.normalize();
    let plane_d = -plane_n.dot(plane_p);
    let ad = line_start.dot(&plane_n);
    let bd = line_end.dot(&plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = line_end - line_start;
    let line_to_intersect = line_start_to_end * t;
    (line_start + line_to_intersect, t)
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
