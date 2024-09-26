use std::slice::from_raw_parts;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};

extern crate image;

/// The possible Anaglyph colors listed [here](https://en.wikipedia.org/wiki/Anaglyph_3D#Anaglyphic_color_channels).
pub enum Coloring {
    RedGreen,
    RedBlue,
    RedCyan,
    Anachrome,
    Mirachrome,
    Trioscopic,
    Colorcode3D,
    MagentaCyan,
}

const PURERED: [f32 ; 4] = [255./255., 0./255., 0./255., 1.];
const PUREGREEN: [f32 ; 4] = [0./255., 255./255., 0./255., 1.];
const PUREBLUE: [f32 ; 4] = [0./255., 0./255., 255./255., 1.];
const PURECYAN: [f32 ; 4] = [0./255., 255./255., 255./255., 1.];
const DARKRED: [f32 ; 4] = [204./255., 0./255., 0./255., 1.];
const CYAN: [f32 ; 4] = [153./255., 204./255., 255./255., 1.];
const PUREMAGENTA: [f32 ; 4] = [255./255., 0./255., 255./255., 1.];
const AMBER: [f32 ; 4] = [255./255., 191./255., 0./255., 1.];
const DARKBLUE: [f32 ; 4] = [0./255., 0./255., 153./255., 1.];

/// Creates the anaglyph from the given file, and saves it in the output file.
/// 
/// * `input_file` - The name of the input file.
/// * `output_file` - The name of the output file.
/// * `offset_x` - The horizontal difference between the two colors.
/// * `offset_y` - The vertical difference between the two colors.
/// * `coloring` - One of the anaglyph [Coloring] couples.
pub fn create(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, offset_x: isize, offset_y: isize, coloring: Coloring) -> Vec<u8> {
    let offset_x = offset_x /2; 
    let offset_y = offset_y /2;
    let direction: &str = {
        if offset_x>=0 {
            if offset_y>=0 { "bottomright" }
            else { "topright" }
        }
        else {
            if offset_y>=0 { "bottomleft" }
            else { "topleft" }
        }
    };
    let direction = direction.to_owned();
    let (color_left, color_right): (_, _) = match coloring {
        Coloring::RedGreen => (PURERED, PUREGREEN),
        Coloring::RedBlue => (PURERED, PUREBLUE),
        Coloring::RedCyan => (PURERED, PURECYAN),
        Coloring::Anachrome => (DARKRED, CYAN),
        Coloring::Mirachrome => (DARKRED, CYAN),
        Coloring::Trioscopic => (PUREGREEN, PUREMAGENTA),
        Coloring::Colorcode3D => (AMBER, DARKBLUE),
        Coloring::MagentaCyan => (PUREMAGENTA, PURECYAN),
    };
    // let img = image::open(input_file).unwrap();
    
    let (in_x, in_y) = img.dimensions();
    let (in_x, in_y) = (in_x as isize, in_y as isize);
    let out_x = in_x - offset_x.abs();
    let out_y = in_y - offset_y.abs();
    let in_buf = img.clone().into_raw();
    let mut out_buf = vec![0u8 ; (out_x * out_y * 4) as usize];
    println!("Direction {}, In {}, Out {}", direction, in_x*in_y * 4, out_x * out_y * 4);

    let mut anaglyph = Anaglyph {
        in_buf,

        offset_x: offset_x.abs() as usize,
        in_x: in_x.abs() as usize,
        out_x: out_x.abs() as usize,
        
        offset_y: offset_y.abs() as usize,
        // in_y: in_y.abs() as usize,
        // out_y: out_y.abs() as usize,

        color_left,
        color_right,

        direction,
    };

    anaglyph.fill(&mut out_buf);

    // image::save_buffer(&output_file, &out_buf, out_x as u32, out_y as u32, image::ColorType::Rgb8).unwrap();
    let mut res = DynamicImage::ImageRgba8(ImageBuffer::from_raw(out_x as u32, out_y as u32, out_buf).unwrap());
    res = res.resize_to_fill(in_x as u32, in_y as u32, image::imageops::FilterType::Nearest);
    res.to_bytes()
}

struct Anaglyph {
    in_buf: Vec<u8>,

    offset_x: usize,
    in_x: usize,
    out_x: usize,

    // in_y: usize,
    offset_y: usize,
    // out_y: usize,

    color_left: [f32 ; 4],
    color_right: [f32 ; 4],

    direction: String,
}

impl Anaglyph {
    fn get(&self, i: usize) -> (u8, u8) {
        if self.direction==String::from("bottomright") {
            let val1 = self.in_buf[i + 4 * (self.offset_y*self.in_x + self.offset_x * (i/(self.out_x * 4) + 1))];
            let val2 = self.in_buf[i + 4 * (self.offset_x * i/(self.out_x * 4))];
            (val1, val2)
        }
        else if self.direction==String::from("topright") {
            let val1 = self.in_buf[i + 4 * (self.offset_x * (i/(self.out_x * 4) + 1))];
            let val2 = self.in_buf[i + 4 * (self.offset_y*self.in_x + self.offset_x * i/(self.out_x * 4))];
            (val1, val2)
        }
        else if self.direction==String::from("bottomleft") {
            let val1 = self.in_buf[i + 4 * (self.offset_y*self.in_x + self.offset_x * i/(self.out_x * 4))];
            let val2 = self.in_buf[i + 4 * (self.offset_x * (i/(self.out_x * 4) + 1))];
            (val1, val2)
        }
        else {
            let val1 = self.in_buf[i + 4 * (self.offset_x * i/(self.out_x * 4))];
            let val2 = self.in_buf[i + 4 * (self.offset_y*self.in_x + self.offset_x * (i/(self.out_x * 4) + 1))];
            (val1, val2)
        }
    }

    fn fill(&mut self, out_buf: &mut Vec<u8>) {
        let mut v1: u8;
        let mut v2: u8;
        for (i, byte) in out_buf.iter_mut().enumerate() {
            (v1, v2) = self.get(i);
            *byte = ((v1 as f32*self.color_left[i%4]) * 0.5 + (v2 as f32*self.color_right[i%4]) * 0.5) as u8;
        }
    }
}