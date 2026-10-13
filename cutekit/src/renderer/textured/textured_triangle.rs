use nalgebra::{Vector2, Vector3};

use crate::nadk::display::Color565;
use crate::renderer2d::elements::Texture;
use crate::renderer2d::renderer::{SCREEN_TILE_HEIGHT, SCREEN_TILE_WIDTH};

fn lerp(a: f32, b: f32, t: f32) -> f32 { (a + (b - a) * t) }
fn mult(tex: Vector3<f32>, size: Vector2<f32>) -> Vector3<f32>
{
	Vector3::new(tex.x * size.x, tex.y * size.y, tex.z)
}

pub fn textured_semiquad(
	frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
	depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
	mut point1: Vector2<i16>,
	mut tex1: Vector3<f32>,
	mut point2: Vector2<i16>,
	mut tex2: Vector3<f32>,
	mut width3: i16,
	mut tex3: Vector3<f32>,
	mut width4: i16,
	mut tex4: Vector3<f32>,
	texture: &Texture,
)
{
	let h = point2.y - point1.y;
	if h <= 0 {
		return;
	}

	let (tw, th) = (texture.width as usize, texture.height as usize);
	let tsize = Vector2::new(texture.width as f32, texture.height as f32);

	let (tl, bl) = (point1.x as f32, point2.x as f32);
	let (tr, br) = ((point1.x + width3) as f32, (point2.x + width4) as f32);
	let (tex_tl, tex_bl) = (mult(tex1, tsize), mult(tex2, tsize));
	let (tex_tr, tex_br) = (mult(tex3, tsize), mult(tex4, tsize));

	let inv_h = 1.0 / h as f32;
	let l_step = (bl - tl) * inv_h;
	let r_step = (br - tr) * inv_h;
	let ltex_step = (tex_bl - tex_tl) * inv_h;
	let rtex_step = (tex_br - tex_tr) * inv_h;

	let y_start = (point1.y as usize).max(0);
	let y_end = (point2.y as usize).min(SCREEN_TILE_HEIGHT - 1);

	for y in y_start..=y_end {
		let dy = y as f32 - point1.y as f32;

		let l = tl + l_step * dy;
		let r = tr + r_step * dy;

		let x_start = (l as usize).max(0);
		let x_end = (r as usize).min(SCREEN_TILE_WIDTH - 1);
		if x_start >= x_end {
			continue;
		}

		let ltex = tex_tl + ltex_step * dy;
		let rtex = tex_tr + rtex_step * dy;
		let tex_step = (rtex - ltex) / (r - l);
		let mut tex = ltex + tex_step * (x_start as f32 - l);

		let row = y * SCREEN_TILE_WIDTH;
		for p in row + x_start..=row + x_end {
			let tz = tex.z as f16;

			if tz < depth_buffer[p] {
				let ptex = Vector2::new(tex.x, tex.y);
				let tx = (ptex.x as usize).min(tw - 1);
				let ty = (ptex.y as usize).min(th - 1);
				frame_buffer[p] = texture.data[ty * tw + tx];
				depth_buffer[p] = tz;
			}
			tex += tex_step;
		}
	}
}

pub fn textured_triangle(
	frame_buffer: &mut [Color565; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
	depth_buffer: &mut [f16; SCREEN_TILE_WIDTH * SCREEN_TILE_HEIGHT],
	mut point1: Vector2<i16>,
	mut tex1: Vector3<f32>,
	mut point2: Vector2<i16>,
	mut tex2: Vector3<f32>,
	mut point3: Vector2<i16>,
	mut tex3: Vector3<f32>,
	texture: &Texture,
)
{
	let mut points = [(point1, tex1), (point2, tex2), (point3, tex3)];
	points.sort_unstable_by(|p1, p2| p1.0.y.cmp(&p2.0.y));
	let [(high_pos, high_tex), (mid_pos, mid_tex), (low_pos, low_tex)] = points;

	let r = (mid_pos.y - high_pos.y) as f32 / (low_pos.y - high_pos.y) as f32;
	let mp_pos = Vector2::new(
		lerp(high_pos.x as f32, low_pos.x as f32, r) as i16,
		mid_pos.y,
	);
	let mp_tex = high_tex.lerp(&low_tex, r);

	let (midl_pos, midl_tex, midr_pos, midr_tex) = if mp_pos.x < mid_pos.x {
		(mp_pos, mp_tex, mid_pos, mid_tex)
	} else {
		(mid_pos, mid_tex, mp_pos, mp_tex)
	};
	let w = midr_pos.x - midl_pos.x;

	textured_semiquad(
		frame_buffer, depth_buffer, high_pos, high_tex, midl_pos, midl_tex, 0, high_tex, w,
		midr_tex, texture,
	);
	textured_semiquad(
		frame_buffer, depth_buffer, midl_pos, midl_tex, low_pos, low_tex, w, midr_tex, 0, low_tex,
		texture,
	);
}
