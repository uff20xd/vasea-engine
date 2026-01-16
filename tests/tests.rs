//extern vasae;
use vasea::*;
use std::{
    io::prelude::*,
    fs,
    num::Wrapping,
};
type Byte = u8;
const SCALE: usize = 4;
const XDIM: usize = SCALE * 16 * 40;
const YDIM: usize = SCALE * 16 * 40;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut thread_pool = ThreadPool::new();
    let input = "./tests/eminem_test.ppm";
    let out = "./out/output.ppm";
    let zoom = 1200.2;
    let x_shift = -0.0;
    let y_shift = 0.64;
    let in_image = Image::read_ppm(input)?;
    let shader = Shader::new(&mandel_brot_shader, zoom, x_shift, y_shift, in_image);
    let image = shader.apply_shader(&mut thread_pool);
    image.write(out);
    
    Ok(())
}

// x, y, zoom, width, height
fn mandel_brot_shader(in_pixel: Pixel, x: usize, y: usize, width: usize, height: usize, zoom: f64,  x_shift: f64, y_shift: f64) -> Pixel {
    if (width*x + y)%(SCALE * SCALE * 5000) == 0 { println!( "Pixel {} out of {}", width*x + y, width * height)}
    let zoom_mult = 1.0 / zoom;
    let x0 = zoom_mult * (((x as f64/width as f64)/ 2.0) - (x as f64/width as f64)) + x_shift;
    let y0 = zoom_mult * (((y as f64/height as f64)/ 2.0) - (y as f64/height as f64)) + y_shift;
    let r_mult: f64 = 0.4 - (0.1_f64 * (x0 + y0 - 1.0)).abs() + 0.2 - (0.1_f64 * (x0 + y0 - 0.0)).abs();
    let g_mult: f64 = 0.4 - (0.1_f64 * (x0 + y0 - 1.0)).abs();
    let b_mult: f64 = 0.1; 
    let max_mult = r_mult.max(g_mult.max(b_mult));
    let (in_r, in_g, in_b) = in_pixel.get_rgb();

    let mut x = 0.0;
    let mut y = 0.0;

    let mut colour: f64 = 0.0;
    let mut n = 0;
    let max_iteration = 20000;

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

    let r = Wrapping(in_r) - Wrapping((colour * r_mult).round() as Byte); //
    let g = Wrapping(in_g) - Wrapping((colour * g_mult).round() as Byte); //
    let b = Wrapping(in_b) - Wrapping((colour * b_mult).round() as Byte); //

    Pixel::new(r.0, g.0, b.0)
}
