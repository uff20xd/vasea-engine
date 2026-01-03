use std::{
    io::prelude::*,
    fs,
};
type Byte = u8;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "out/output.ppm";
    let mut output = fs::File::create(name)?;

    let scale = 4;
    let width: usize = 16*40*scale;
    let height: usize = 16*40*scale;
    let size: Vec<u8> = format!("{} {}\n", width, height).bytes().collect();
    let zoom = 1200.2 as f64;
    let x_shift = -0.0 as f64;
    let y_shift = 0.64 as f64;

    _ = output.write(&(b"P6\n")[..]);
    _ = output.write(&size[..]);
    _ = output.write(&(b"255\n")[..]);
    let zoom_mult = 1.0 / zoom ; // scale as f64;

    let mut current_pixel = 0;

    for i in 0..height {
        for j in 0..width {

        }
    }


    println!("Generating File: {}", name);
    Ok(())
}

// x, y, zoom, width, height
fn mandel_brot_shader(x: usize, y: usize, zoom: f64, width: usize, height: usize) -> Pixel {
    current_pixel += 1;
    if (current_pixel % (scale * scale * 5000)) == 0 { println!( "Pixel {} out of {}", current_pixel, width * height)}
    let x0 = zoom_mult * (((j as f64/width as f64)/ 2.0) - (j as f64/width as f64)) + x_shift;
    let y0 = zoom_mult * (((i as f64/height as f64)/ 2.0) - (i as f64/height as f64)) + y_shift;
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


}
