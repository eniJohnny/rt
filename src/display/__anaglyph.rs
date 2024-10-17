use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use web_sys::wasm_bindgen::UnwrapThrowExt;

use crate::ANAGLYPH_OFFSET;

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

const PURERED: [f32 ; 3] = [255./255., 0./255., 0./255.];
const PUREGREEN: [f32 ; 3] = [0./255., 255./255., 0./255.];
const PUREBLUE: [f32 ; 3] = [0./255., 0./255., 255./255.];
const PURECYAN: [f32 ; 3] = [0./255., 255./255., 255./255.];
const DARKRED: [f32 ; 3] = [204./255., 0./255., 0./255.];
const CYAN: [f32 ; 3] = [153./255., 204./255., 255./255.];
const PUREMAGENTA: [f32 ; 3] = [255./255., 0./255., 255./255.];
const AMBER: [f32 ; 3] = [255./255., 191./255., 0./255.];
const DARKBLUE: [f32 ; 3] = [0./255., 0./255., 153./255.];

/// Creates the anaglyph from the given file, and saves it in the output file.
/// 
/// * `input_file` - The name of the input file.
/// * `output_file` - The name of the output file.
/// * `offset_x` - The horizontal difference between the two colors.
/// * `offset_y` - The vertical difference between the two colors.
/// * `coloring` - One of the anaglyph [Coloring] couples.
pub fn create(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> RgbaImage {
    let offset_x = ANAGLYPH_OFFSET_X;
    let offset_y = ANAGLYPH_OFFSET_Y;
    let coloring = Coloring::RedCyan;
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
    let out_x = in_x - offset_x;
    let out_y = in_y - offset_y;
    let in_buf = img.clone().into_raw();
    let (insize, outsize) = ((in_x*in_y * 3) as usize, (out_x * out_y * 3) as usize);
    let real_offset_x = offset_x + (offset_x as f32 * 16_f32 / 9_f32).ceil() as isize;
    let real_offset_y = offset_y + (offset_y as f32 * 9_f32 / 16_f32).ceil() as isize;
    let mut out_buf = vec![0u8 ; in_buf.len()];

    println!("Direction {}, In {} -> {}x{}, Out {} -> {}x{}", direction, insize, insize/2700, insize/2700*9/16, outsize, outsize/2700, outsize/2700*9/16);
    println!("Real offset x {}, y {}", real_offset_x, real_offset_y);

    let mut anaglyph = Anaglyph {
        in_buf,
        insize,

        offset_x: offset_x as usize,
        in_x: in_x as usize,
        out_x: out_x as usize,
        
        offset_y: offset_y as usize,
        // in_y: in_y.abs() as usize,
        // out_y: out_y.abs() as usize,

        color_left,
        color_right,

        direction,
    };

    anaglyph.fill(&mut out_buf);

    let res = RgbaImage::from_raw(in_x as u32, in_y as u32, out_buf);
    res.unwrap()


    // image::save_buffer(&output_file, &out_buf, out_x as u32, out_y as u32, image::ColorType::Rgb8).unwrap();
}

struct Anaglyph {
    in_buf: Vec<u8>,
    insize: usize,

    offset_x: usize,
    in_x: usize,
    out_x: usize,

    // in_y: usize,
    offset_y: usize,
    // out_y: usize,

    color_left: [f32 ; 3],
    color_right: [f32 ; 3],

    direction: String,
}

impl Anaglyph {
    fn get(&self, i: usize, counter: &mut u32) -> (u8, u8, u32) {
        if self.direction==String::from("bottomright") {
            let x_limit = self.out_x * 3;
            let x_max = self.in_x * 3;
            let offset = self.offset_x * 3;

            // in_x = 1600
            // in_y = 900
            // x_max = 4800
            // x_limit = 1590

            // println!("{}:{}", i, cnt);
            // if i % x_max == 0 && i != 0 {
            //     return (0, 0, *counter + 1);
            // } else if i % x_max >= x_limit {
            //     return (0, 0, *counter);
            // }

            let x = i - offset * *counter as usize;
            let index1 = x + 3 * (self.offset_y*self.in_x + self.offset_x * (x/(self.out_x * 3) + 1));
            let index2 = x + 3 * (self.offset_x * x/(self.out_x * 3));
            // println!("index 1: {}, index 2: {}", index1, index2);

            if (index1 >= self.in_buf.len()) || (index2 >= self.in_buf.len()) {
                return (0, 0, *counter);
            }
            if i != 0 && i % x_max == 0 {
                return (0, 0, *counter + 1);
            } else if i != 0 && i % x_max >= x_limit {
                return (0, 0, *counter);
            }

            let val1 = self.in_buf[index1];
            let val2 = self.in_buf[index2];
            (val1, val2, *counter)
        }
        else if self.direction==String::from("topright") {
            let val1 = self.in_buf[i + 3 * (self.offset_x * (i/(self.out_x * 3) + 1))];
            let val2 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * i/(self.out_x * 3))];
            (val1, val2, *counter)
        }
        else if self.direction==String::from("bottomleft") {
            let val1 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * i/(self.out_x * 3))];
            let val2 = self.in_buf[i + 3 * (self.offset_x * (i/(self.out_x * 3) + 1))];
            (val1, val2, *counter)
        }
        else {
            let val1 = self.in_buf[i + 3 * (self.offset_x * i/(self.out_x * 3))];
            let val2 = self.in_buf[i + 3 * (self.offset_y*self.in_x + self.offset_x * (i/(self.out_x * 3) + 1))];
            (val1, val2, *counter)
        }
    }

    fn fill(&mut self, out_buf: &mut Vec<u8>) {
        let mut v1: u8;
        let mut v2: u8;
        // for (i, byte) in out_buf.iter_mut().enumerate() {
        //     (v1, v2) = self.get(i);
        //     *byte = ((v1 as f32*self.color_left[i%3]) * 0.5 + (v2 as f32*self.color_right[i%3]) * 0.5) as u8;
        // }
        let mut y_counter = 0;

        for i in 0..self.insize {

            (v1, v2, y_counter) = self.get(i, &mut y_counter);
            out_buf[i] = ((v1 as f32*self.color_left[i%3]) * 0.5 + (v2 as f32*self.color_right[i%3]) * 0.5) as u8;
        }
    }
}
