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

#[derive(Default, Clone, Copy)]
struct Pixel {
    r: Byte,
    g: Byte,
    b: Byte,
    // pub inner: Mutex<InnerPixel>
}

#[derive(Default, Clone, Copy)]
struct InnerPixel {
    r: Byte,
    g: Byte,
    b: Byte,
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

pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    image: Mutex<[[Pixel; WIDTH]; HEIGHT]>,
    file: fs::File,
}

pub struct Shader<const WIDTH: usize, const HEIGHT: usize, F> 
    where F: Fn(usize, usize, f64, usize, usize) -> Pixel {
    // x, y, zoom, width, height
    zoom: f64
    pixel_fn: F,
    image: Image<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize, F> Shader<WIDTH, HEIGHT, F> 
    where F: Fn(usize, usize, f64, usize, usize) -> Pixel {

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
        let image = self.image.get();
        let size: Vec<u8> = format!("{} {}\n", width, height).bytes().collect();
        _ = self.file.write(&(b"P6\n")[..]);
        _ = self.file.write(&size[..]);
        _ = self.file.write(&(b"255\n")[..]);

        for x in self.width {
            for y in self.height {
                let pixel = image[x][y];
                self.file.write(&[pixel.r, pixel.g, pixel.b])
            }
        }
    }
}

impl<F> Task<F>
    where F: Fn(usize, usize, usize, usize, usize) -> Pixel {
    pub fn new(
        function: F,
        x: usize,
        y: usize,
        zoom: usize,
        width: usize,
        height: usize,
    ) -> Task {
        Self {
            function,
            x,
            y,
            zoom,
            width,
            height,
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
