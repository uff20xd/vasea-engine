use std::{
    io::prelude::*,
    fs,
    sync::Mutex,
};
use std::ffi::OsStr;
use std::path::Path;
type Byte = u8;

// PPM Format:
// "P6" \n
// $width $height\n
// $max_colour_component_value\n
// (($r as byte)($g as byte)($b as byte))+
macro_rules! generate_task {
    () => {}
}

#[derive(Default, Clone, Copy)]
struct Pixel {
    r: Byte,
    g: Byte,
    b: Byte,
    // pub inner: Mutex<InnerPixel>
}


struct Task<F> 
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel {
    function: F,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    zoom: f64,
    x_shift: f64,
    y_shift: f64,
}

pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    image: Mutex<[[Pixel; WIDTH]; HEIGHT]>,
    file: fs::File,
}

pub struct Shader<const WIDTH: usize, const HEIGHT: usize, F> 
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel {
    // x, y, zoom, width, height
    zoom: f64
    pixel_fn: F,
    image: Image<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize, F> Shader<WIDTH, HEIGHT, F> 
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel {

    pub fn new() -> Self { todo!( ) }
    pub fn get_task(x: usize, y: usize) -> Task<F> { 

        let task = Task::new(
            self.pixel_fn.clone(),
            x,
            y,
        );

        task
    }
    pub fn apply_shader(self) -> Self {
        self
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, F> Image<WIDTH, HEIGHT> {

    pub fn new<T>(file_name: T) -> Self
    where T: AsRef<Path> {
        Self {
            image: Mutex::new([[Pixel::default(); WIDTH]; HEIGHT]),
            file: fs::File::create(file_name).expect(&format!("File couldnt be created in Line: {}", line!())),
        }
    }
    pub fn write(&self) -> Result<(), ()> {
        let image = self.image.lock().unwrap();

        let size: Vec<u8> = format!("{} {}\n", width, height).bytes().collect();
        _ = self.file.write(&(b"P6\n")[..]);
        _ = self.file.write(&size[..]);
        _ = self.file.write(&(b"255\n")[..]);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let pixel = image[x][y];
                self.file.write(&[pixel.r, pixel.g, pixel.b]);
            }
        }
        Ok(())
    }
}

impl<F> Task<F>
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel {
    pub fn new(
        function: F,
        x: usize,
        y: usize,
        zoom: usize,
        width: usize,
        height: usize,
        x_shift: usize,
        y_shift: usize,
    ) -> Task {
        Self {
            function,
            x,
            y,
            zoom,
            width,
            height,
            x_shift,
            y_shift,
        }
    }

    #[inline(always)]
    fn execute(&self) -> Pixel {
        (self.function)(
            self.x,
            self.y,
            self.zoom,
            self.width,
            self.height,
            self.x_shift,
            self.y_shift,
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn todo() {
        todo!()
    }
}
