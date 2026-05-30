calc_use!(alloc::vec::Vec);

use libm::roundf;
use nalgebra::Vector2;

use crate::nadk::display::{COLOR_BLACK, COLOR_BLUE, COLOR_RED, Color565};

#[inline]
fn put_pixel(buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, x: isize, y: isize, color: Color565)
{
    if x < 0 || x >= buffer_width {return ;}
    if y < 0 || y >= buffer_height {return ;}
    buffer[(x + y * buffer_width) as usize] = color;
}

#[inline]
fn get_pixel(buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, x: isize, y: isize) -> Color565
{
    if x < 0 || x >= buffer_width {return COLOR_BLACK;}
    if y < 0 || y >= buffer_height {return COLOR_BLACK;}
    buffer[(x + y * buffer_width) as usize]
}

// Based on the Midpoint algorithm on Wikipedia: https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
pub fn circle(r: f32, center: Vector2<isize>, buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, color: Color565)
{
    let mut t1 = r / 16.0;
    let mut x = r;
    let mut y = 0.0;
    while x >= y {
        put_pixel(buffer, buffer_width, buffer_height, center.x + x as isize, center.y + y as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x - x as isize, center.y + y as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x + x as isize, center.y - y as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x - x as isize, center.y - y as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x + y as isize, center.y + x as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x - y as isize, center.y + x as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x + y as isize, center.y - x as isize, color);
        put_pixel(buffer, buffer_width, buffer_height, center.x - y as isize, center.y - x as isize, color);
        y += 1.0;
        t1 += y;
        let t2 = t1 - x;
        if t2 >= 0.0
        {
            t1 = t2;
            x -= 1.0;
        }
    }
}

#[inline]
fn put_horizontal_line_smooth(y: isize, x_start: isize, width: isize, buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, color: Color565)
{
    let mut x = x_start;
    let stop = x_start + width;
    while x < stop {
        put_pixel(buffer, buffer_width, buffer_height, x, y, color);
        x += 1;
    }
    blend_pixel(buffer, buffer_width, buffer_height, x_start - 1, y, color);
    blend_pixel(buffer, buffer_width, buffer_height, stop, y, color);
}

#[inline]
fn put_horizontal_line(y: isize, x_start: isize, width: isize, buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, color: Color565)
{
    let mut x = x_start;
    let stop = x_start + width;
    while x < stop {
        put_pixel(buffer, buffer_width, buffer_height, x, y, color);
        x += 1;
    }
}

pub fn rectangle(x: isize, y: isize, width: isize, height: isize, buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, color: Color565)
{
    for xx in x..(x + width)
    {
        for yy in y..(y + height)
        {
            put_pixel(buffer, buffer_width, buffer_height, xx, yy, color);
        }
    }
}

#[inline]
fn mix_colors(a: Color565, b: Color565) -> Color565
{
    let a_comp = a.get_components();
    let b_comp = b.get_components();
    Color565::new((a_comp.0 + b_comp.0) / 2, (a_comp.1 + b_comp.1) / 2, (a_comp.2 + b_comp.2) / 2)
}

#[inline]
fn blend_pixel(buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, x: isize, y: isize, color: Color565)
{
    let color = mix_colors(color, get_pixel(buffer, buffer_width, buffer_height, x, y));
    put_pixel(buffer, buffer_width, buffer_height, x, y, color);
}

pub fn rounded_rectangle(r: f32, center: Vector2<isize>, width: isize, height: isize, buffer: &mut [Color565], buffer_width: isize, buffer_height: isize, color: Color565)
{
    let mut t1 = r / 16.0;
    let mut x = r;
    let mut y = 0.0;
    let half_width = width / 2;
    let half_height = height / 2;
    let r_isize = r as isize;
    while x >= y {
        let rounded_x = roundf(x) as isize;
        let rounded_y = roundf(y) as isize;
        // Fill the rounded parts
        if rounded_x == r_isize
        {
            put_horizontal_line(center.y - half_height - rounded_y + r_isize, center.x - half_width - rounded_x + r_isize, width + 2 * rounded_x - 2 * r_isize, buffer, buffer_width, buffer_height, color);
            put_horizontal_line(center.y + half_height + rounded_y - r_isize - 1, center.x - half_width - rounded_x + r_isize, width + 2 * rounded_x - 2 * r_isize, buffer, buffer_width, buffer_height, color);

        } else {
            put_horizontal_line_smooth(center.y - half_height - rounded_y + r_isize, center.x - half_width - rounded_x + r_isize, width + 2 * rounded_x - 2 * r_isize, buffer, buffer_width, buffer_height, color);
            put_horizontal_line_smooth(center.y + half_height + rounded_y - r_isize - 1, center.x - half_width - rounded_x + r_isize, width + 2 * rounded_x - 2 * r_isize, buffer, buffer_width, buffer_height, color);

        }
        if rounded_y == r_isize {
            put_horizontal_line(center.y - half_height - rounded_x + r_isize, center.x - half_width - rounded_y + r_isize, width + 2 * rounded_y - 2 * r_isize - 1, buffer, buffer_width, buffer_height, color);
            put_horizontal_line(center.y + half_height + rounded_x - r_isize - 1, center.x - half_width - rounded_y + r_isize, width + 2 * rounded_y - 2 * r_isize - 1, buffer, buffer_width, buffer_height, color);
        }
        else {
            put_horizontal_line_smooth(center.y - half_height - rounded_x + r_isize, center.x - half_width - rounded_y + r_isize, width + 2 * rounded_y - 2 * r_isize, buffer, buffer_width, buffer_height, color);
            put_horizontal_line_smooth(center.y + half_height + rounded_x - r_isize - 1, center.x - half_width - rounded_y + r_isize, width + 2 * rounded_y - 2 * r_isize, buffer, buffer_width, buffer_height, color);
        }
        y += 1.0;
        t1 += y;
        let t2 = t1 - x;
        if t2 >= 0.0
        {
            t1 = t2;
            x -= 1.0;
        }
    }
    rectangle(center.x - half_width, center.y - (half_height - r_isize), width, height - r_isize * 2, buffer, buffer_width, buffer_height, color);
}
