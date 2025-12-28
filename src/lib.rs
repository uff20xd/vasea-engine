use std::{
    io::prelude::*,
    fs,
};
use std::ffi::OsStr;
use std::path::Path;
type Byte = u8;

// PPM Format:
// "P6" \n
// $width $height\n
// $max_colour_component_value\n
// (($r as byte)($g as byte)($b as byte))+

#[derive(Default, Clone, Copy)]
struct Pixel {
    r: Byte,
    g: Byte,
    b: Byte,
}

struct Image<const WIDTH: usize, const HEIGHT: usize, F>
    where F: Fn(usize, usize, usize, usize, usize) -> Pixel {
    image: [[Pixel; WIDTH]; HEIGHT],
    // x, y
    next_pixel: (usize, usize),
    file_name: fs::File,
    // x, y, zoom, width, height
    pixel_fn: F,
}

impl<const WIDTH: usize, const HEIGHT: usize, F> Image<WIDTH, HEIGHT, F> 
    where F: Fn(usize, usize, usize, usize, usize) -> Pixel {

    pub fn new<T>(file_name: T, pixel_fn: F) -> Self
    where T: AsRef<Path> {
        Self {
            image: [[Pixel::default(); WIDTH]; HEIGHT],
            next_pixel: (0, 0),
            file_name: fs::File::create(file_name).expect(&format!("File couldnt be created in Line: {}", line!())),
            pixel_fn,
        }
    }
    pub fn get_task() -> Task<F> { todo!() }
}

struct Task<F> 
    where F: Fn(usize, usize, usize, usize, usize) -> Pixel {
    function: F,
    x: usize,
    y: usize,
    zoom: usize,
    width: usize,
    height: usize,
}

impl<F> Task<F>
    where F: Fn(usize, usize, usize, usize, usize) -> Pixel {
    #[inline(always)]
    fn execute(&self) -> Pixel {
        (self.function)(
            self.x,
            self.y,
            self.zoom,
            self.width,
            self.height,
        )
    }
}
