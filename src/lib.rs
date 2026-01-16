use std::{
    io::prelude::*,
    fs,
    sync::Mutex,
    sync::Arc,
    ffi::OsStr,
    path::PathBuf,
    path::Path,
};
type Byte = u8;
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
    where F: Fn(Pixel, usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {
    function: &'static F,
    pixel: Pixel,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    zoom: f64,
    x_shift: f64,
    y_shift: f64,
}

#[derive(Debug)]
pub struct Image {
    image: Arc<Mutex<Vec<Byte>>>,
    dimensions: (usize, usize),
}

pub struct ThreadPool {
    pool: Vec<()>,
}

pub struct Shader<F> 
    where F: Fn(Pixel, usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {
    // x, y, zoom, width, height
    pixel_fn: &'static F,
    zoom: f64,
    x_shift: f64,
    y_shift: f64,
    image: Arc<Image>,
}

impl Pixel {
    pub fn new(r: Byte, g: Byte, b: Byte) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
    pub fn get_rgb(&self) -> (Byte, Byte, Byte){
        (self.r, self.g, self.b)
    }
}
impl<F> Task<F>
    where F: Fn(Pixel, usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {
    pub fn new(
        function: &'static F,
        pixel: Pixel,
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
            pixel,
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
            self.pixel,
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

impl Image {
    pub fn new(width: usize, height: usize, base: Byte) -> Self {
        Self {
            image: Mutex::new(vec![base; width * height * 3]).into(),
            dimensions: (width, height),
        }
    }
    pub fn write<T>(&self, file_name: T) -> Result<(), Box<dyn std::error::Error>> 
    where T: AsRef<Path> {
        let image = self.image.lock().unwrap();
        let mut file = fs::File::create(file_name.as_ref())?;
        let (width, height) = self.dimensions;

        let size: Vec<u8> = format!("{} {}\n", width, height).bytes().collect();
        _ = file.write(&(b"P6\n")[..]);
        _ = file.write(&size[..]);
        _ = file.write(&(b"255\n")[..]);

        file.write(&image[..]);
        Ok(())
    }
    pub fn read_ppm<T>(file_name: T) -> Result<Self, Box<dyn std::error::Error>>
    where T: AsRef<Path> {
        let mut file = fs::read(file_name.as_ref())?;
        let mut index = 3;
        let mut raw_numbers = String::new();
        loop {
            if file[index] == 10 { break; }
            raw_numbers.push(file[index] as char);
            index += 1;
        }
        index += 5;
        let image_data = Vec::from(&file[index..]);
        let mut raw_number_split = raw_numbers.split_whitespace();
        let raw_width = raw_number_split.next().expect("Some");
        let raw_height = raw_number_split.next().expect("Some");
        dbg!(&raw_width);
        dbg!(&raw_height);
        let width: usize = raw_width.parse()?;
        let height: usize = raw_height.parse()?;
        Ok(Self {
            image: Mutex::new(image_data).into(),
            dimensions: (width, height),
        })
    }
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
        }
    }
}

impl<F> Shader<F> 
    where F: Fn(Pixel, usize, usize, usize, usize, f64, f64, f64) -> Pixel + 'static {

    pub fn new(
        pixel_fn: &'static F,
        zoom: f64,
        x_shift: f64,
        y_shift: f64,
        image: Image<>,
    ) -> Self { 
        Self {
            pixel_fn,
            zoom,
            x_shift,
            y_shift,
            image: image.into(),
        }
    }
    pub fn get_task(&self, x: usize, y: usize, pixel: Pixel) -> Task<F> { 
        let (width, height) = self.image.dimensions;
        let task = Task::new(
            self.pixel_fn,
            pixel,
            x,
            y,
            width,
            height,
            self.zoom,
            self.x_shift,
            self.y_shift,
        );

        task
    }
    pub fn apply_shader(self, _thread_pool: &mut ThreadPool) -> Arc<Image> {
        {
            let mut image = self.image.image.lock().unwrap();
            let mut x = 0;
            let mut y = 0;
            let (width, height) = self.image.dimensions;
            for x in 0..width {
                for y in 0..height {
                    let pixel = Pixel::new(image[(y*width + x)*3],
                    image[(y*width + x)*3 + 1],
                    image[(y*width + x)*3 + 2]);
                    let task = self.get_task(x, y, pixel);
                    let (r,g,b) = task.execute().get_rgb();
                    image[(y*width + x)*3] = r;
                    image[(y*width + x)*3 + 1] = g;
                    image[(y*width + x)*3 + 2] = b;
                }
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
