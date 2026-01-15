use std::{
    io::prelude::*,
    fs,
    sync::Mutex,
};
use std::ffi::OsStr;
use std::path::Path;
type Byte = u8;
use std::path::PathBuf;
// PPM Format:
// "P6" \n
// $width $height\n
// $max_colour_component_value\n
// (($r as byte)($g as byte)($b as byte))+
macro_rules! generate_task {
    () => {}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Pixel {
    r: Byte,
    g: Byte,
    b: Byte,
    // pub inner: Mutex<InnerPixel>
}

struct Task<F> 
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {
    function: &'static F,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    zoom: f64,
    x_shift: f64,
    y_shift: f64,
}

#[derive(Debug)]
pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    image: Mutex<[[Pixel; WIDTH]; HEIGHT]>,
    path: PathBuf,
}

pub struct ThreadPool {
    pool: Vec<()>,
}

pub struct Shader<const WIDTH: usize, const HEIGHT: usize, F> 
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {
    // x, y, zoom, width, height
    pixel_fn: &'static F,
    zoom: f64,
    x_shift: f64,
    y_shift: f64,
    image: Image<WIDTH, HEIGHT>,
}

impl Pixel {
    pub fn new(r: Byte, g: Byte, b: Byte) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}
impl<F> Task<F>
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel {
    pub fn new(
        function: &'static F,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        zoom: f64,
        x_shift: f64,
        y_shift: f64,
    ) -> Task<F> {
        Self {
            function,
            x,
            y,
            width,
            height,
            zoom,
            x_shift,
            y_shift,
        }
    }

    #[inline(always)]
    fn execute(&self) -> Pixel {
        (self.function)(
            self.x,
            self.y,
            self.width,
            self.height,
            self.zoom,
            self.x_shift,
            self.y_shift,
        )
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    pub fn new<T>(file_name: T) -> Self
    where T: AsRef<Path> {
        Self {
            image: Mutex::new([[Pixel::default(); WIDTH]; HEIGHT]),
            path: file_name.as_ref().into(),
        }
    }
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let image = self.image.lock().unwrap();
        let mut file = fs::File::create(&self.path)?;

        let size: Vec<u8> = format!("{} {}\n", WIDTH, HEIGHT).bytes().collect();
        _ = file.write(&(b"P6\n")[..]);
        _ = file.write(&size[..]);
        _ = file.write(&(b"255\n")[..]);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let pixel = image[x][y];
                file.write(&[pixel.r, pixel.g, pixel.b]);
            }
        }
        Ok(())
    }
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, F> Shader<WIDTH, HEIGHT, F> 
    where F: Fn(usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {

    pub fn new(
        pixel_fn: &'static F,
        zoom: f64,
        x_shift: f64,
        y_shift: f64,
        image: Image<WIDTH, HEIGHT>,
    ) -> Self { 
        Self {
            pixel_fn,
            zoom,
            x_shift,
            y_shift,
            image,
        }
    }
    pub fn get_task(&self, x: usize, y: usize) -> Task<F> { 
        let task = Task::new(
            self.pixel_fn,
            x,
            y,
            WIDTH,
            HEIGHT,
            self.zoom,
            self.x_shift,
            self.y_shift,
        );

        task
    }
    pub fn apply_shader(self, _thread_pool: &mut ThreadPool) -> Image<WIDTH, HEIGHT> {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let task = self.get_task(x, y);
                let mut image = self.image.image.lock().unwrap();
                image[x][y] = task.execute();
            }
        }
        self.image
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn todo() {
        todo!()
    }
}
