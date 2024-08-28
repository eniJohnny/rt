use image::{Rgb, Rgba, RgbaImage};

use crate::ANAGLYPH_OFFSET;

extern crate image;

struct Anaglyph {
    in_buf: Vec<u8>,

    offset: usize,
    in_x: usize,
    out_x: usize,

    color_left: [f32 ; 3],
    color_right: [f32 ; 3],
}

impl Anaglyph {
    fn new(in_buf: Vec<u8>, offset: usize, in_x: usize, out_x: usize) -> Anaglyph {
        Anaglyph {
            in_buf,
            offset,
            in_x,
            out_x,
            color_left: [1., 0., 0.],
            color_right: [0., 1., 1.],
        }
    }
    fn get(&self, i: usize) -> (u8, u8) {
        let val1 = self.in_buf[i + 3 * (self.offset*self.in_x + self.offset * (i/(self.out_x * 3) + 1))];
        let val2 = self.in_buf[i + 3 * (self.offset * i/(self.out_x * 3))];
        (val1, val2)
    }

    fn fill(&mut self, out_buf: &mut Vec<u8>) {
        let mut v1: u8;
        let mut v2: u8;
        for (i, byte) in out_buf.iter_mut().enumerate() {
            (v1, v2) = self.get(i);
            *byte = ((v1 as f32*self.color_left[i%3]) * 0.5 + (v2 as f32*self.color_right[i%3]) * 0.5) as u8;
        }
    }
}

// pub fn create(img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Option<image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>> {
//     let offset = ANAGLYPH_OFFSET / 2; 
    
//     let (in_x, in_y) = img.dimensions();
//     let (in_x, in_y) = (in_x as isize, in_y as isize);
//     let out_x = in_x - offset.abs();
//     let out_y = in_y - offset.abs();
//     let in_buf = img.clone().into_raw();
//     let mut out_buf = vec![0u8 ; (out_x * out_y * 3) as usize];

//     let mut anaglyph = Anaglyph::new(in_buf, offset as usize, in_x as usize, out_x as usize);

//     anaglyph.fill(&mut out_buf);

//     match image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(out_x as u32, out_y as u32, out_buf) {
//         Some(output) => Some(output),
//         None => None,
//     }

// }

pub fn create(image: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> RgbaImage {
    let (width, height) = image.dimensions();
    let shift = ANAGLYPH_OFFSET;

    let mut anaglyph = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {

            // Calculate the shifted positions
            let left_x = if x >= shift { x - shift } else { 0 };
            let right_x = if x + shift < width { x + shift } else { width - 1 };

            // Get the shifted pixels
            let left_pixel = image.get_pixel(left_x, y);
            let right_pixel = image.get_pixel(right_x, y);

            // Create the pseudo-anaglyph pixel
            let anaglyph_pixel = Rgba([
                left_pixel[0],  // Red channel from the left-shifted pixel
                right_pixel[1], // Green channel from the right-shifted pixel
                right_pixel[2], // Blue channel from the right-shifted pixel
                255,            // Alpha channel
            ]);

            anaglyph.put_pixel(x, y, anaglyph_pixel);
        }
    }

    anaglyph
}