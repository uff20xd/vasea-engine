use std::{
    io::prelude::*,
    fs,
};
type Byte = u8;

struct Pixel {
    r: Byte,
    g: Byte,
    b: Byte,
}

struct Image<const WIDTH: usize, const HEIGHT: usize> {
    image: [[Pixel; WIDTH]; HEIGHT],
    file_name: fs::File,

}

// PPM Format:
// "P6" \n
// $width $height\n
// $max_colour_component_value\n
// (($r as byte)($g as byte)($b as byte))+
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut name = "out/output.ppm";
    let mut output = fs::File::create(name)?;

    let scale = 6;
    let width: usize = 16*40*scale;
    let height: usize = 16*40*scale;
    let size: Vec<u8> = format!("{} {}\n", width, height).bytes().collect();
    let zoom = 11.6 as f64;
    let x_shift = -0.2 as f64;
    let y_shift = 8.0 as f64;


    _ = output.write(&(b"P6\n")[..]);
    _ = output.write(&size[..]);
    _ = output.write(&(b"255\n")[..]);
    let zoom_mult = 1.0 / zoom ; // scale as f64;
    let actual_shift_x = x_shift * zoom_mult;
    let actual_shift_y = y_shift * zoom_mult;

    let mut current_pixel = 0;

    for i in 0..height {
        for j in 0..width {
            current_pixel += 1;
            if (current_pixel % (scale * scale * 5000)) == 0 { println!( "Pixel {} out of {}", current_pixel, width * height)}
            let x0 = zoom_mult * (((j as f64/width as f64)/ 2.0) - (j as f64/width as f64)) + actual_shift_x;
            let y0 = zoom_mult * (((i as f64/height as f64)/ 2.0) - (i as f64/height as f64)) + actual_shift_y;
            let r_mult: f64 = 0.2 - (0.1_f64 * (x0 + y0 - 1.0)).abs() + 0.06;
            let g_mult: f64 = 0.2 - (0.1_f64 * (x0 + y0 - 1.0)).abs();
            let b_mult: f64 = 0.0; 
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

            if colour < 30.0 {
                colour += 120.0;
            }

            let r = (colour * r_mult).round() as Byte; //
            let g = (colour * g_mult).round() as Byte; //
            let b = (colour * b_mult).round() as Byte; //


            _ = output.write(&[r, g, b]);
        }
    }


    println!("Generating File: {}", name);
    Ok(())
}
