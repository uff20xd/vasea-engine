//extern vasae;
use vasea::*;
use std::{
    io::prelude::*,
    fs,
};
type Byte = u8;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let thread_pool = ThreadPool::new();
    let zoom = 0.0;
    let x_shift = 0.0;
    let y_shift = 0.0;
    let shader = Shader::new(&mandel_brot_shader, zoom, x_shift, y_shift, Image::new("output.ppm")); 
    let image = shader.apply_shader();
    image.write();
    Ok(())
}

// x, y, zoom, width, height
fn mandel_brot_shader(x: usize, y: usize, width: usize, height: usize, zoom: f64,  x_shift: f64, y_shift: f64) -> Pixel {
    // if (% (scale * scale * 5000)) == 0 { println!( "Pixel {} out of {}", current_pixel, width * height)}
    let zoom_mult = 1.0 / zoom;
    let x0 = zoom_mult * (((x as f64/width as f64)/ 2.0) - (x as f64/width as f64)) + x_shift;
    let y0 = zoom_mult * (((y as f64/height as f64)/ 2.0) - (y as f64/height as f64)) + y_shift;
    let r_mult: f64 = 0.2 - (0.1_f64 * (x0 + y0 - 1.0)).abs() + 0.2 - (0.1_f64 * (x0 + y0 - 0.0)).abs();
    let g_mult: f64 = 0.2 - (0.1_f64 * (x0 + y0 - 1.0)).abs();
    let b_mult: f64 = 0.8; 
    let max_mult = r_mult.max(g_mult.max(b_mult));

    let mut x = 0.0;
    let mut y = 0.0;

    let mut colour: f64 = 0.0;
    let mut n = 0;
    let max_iteration = 1000;

    while (x*x + y*y <= 2.0*2.0) && (n < max_iteration) {
        let x_temp = x*x - y*y + x0;

        y = 2.0*x*y + y0;
        x = x_temp;

        n += 1;
    }

    //colour = (255 * (n/max_iteration)) as f64 / max_mult;
    colour = n as f64;

    //if colour < 10.0 {
    //    colour += 120.0;
    //}

    let r = (colour * r_mult).round() as Byte; //
    let g = (colour * g_mult).round() as Byte; //
    let b = (colour * b_mult).round() as Byte; //

    Pixel::new(r, g, b)
}
