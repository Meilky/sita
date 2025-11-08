mod chars;
mod input;
mod px;

use image::{ImageReader, Rgb, RgbImage};

use crate::chars::{FONT_SIZE_HEIGHT, FONT_SIZE_WIDTH, FONT8X8, has_px_at};
use crate::input::Args;
use crate::px::Px;

fn get_char_x_y(scaled_font_height: u32, scaled_font_width: u32, x: u32, y: u32) -> (usize, usize) {
    let char_x: usize = (x / scaled_font_width) as usize;
    let char_y: usize = (y / scaled_font_height) as usize;

    (char_x, char_y)
}

fn gradient_to_char_idx(gradient: u8) -> usize {
    let chars_idx: [usize; 10] = [0, 14, 26, 13, 29, 11, 10, 3, 5, 32];
    let steps = chars_idx.len();

    let idx = (gradient as usize * steps) / 256;
    chars_idx[idx]
}

fn main() {
    let args = Args::from_args();

    let img = ImageReader::open(args.input_file_path)
        .unwrap()
        .decode()
        .unwrap();

    let rgb8_img = img.as_rgb8();

    if rgb8_img.is_none() {
        panic!(
            "sry bro, can't read that image for some reason, maybe cause it has a alpha layer or some shit, idk"
        );
    }

    let buf = rgb8_img.unwrap();

    let width = img.width();
    let height = img.height();

    let scaled_font_height: u32 = FONT_SIZE_HEIGHT as u32 * args.scale as u32;
    let scaled_font_width: u32 = FONT_SIZE_WIDTH as u32 * args.scale as u32;

    let mut nb_columns: u32 = width / scaled_font_width;
    let nb_columns_rest: u32 = width % scaled_font_width;

    if nb_columns_rest > 0 {
        nb_columns += 1;
    }

    let mut nb_lines: u32 = height / scaled_font_height;
    let nb_lines_rest: u32 = height % scaled_font_height;

    if nb_lines_rest > 0 {
        nb_lines += 1;
    }

    let mut char_px: Vec<Px> = Vec::with_capacity((nb_lines * nb_columns) as usize);

    char_px.resize_with((nb_lines * nb_columns) as usize, || Px::new());

    for (x, y, px) in buf.enumerate_pixels() {
        let (char_x, char_y) = get_char_x_y(scaled_font_height, scaled_font_width, x, y);

        char_px[char_x + (char_y * nb_columns as usize)].add_px(px.0[0], px.0[1], px.0[2]);
    }

    let mut char_px_x: u32 = 0;
    let mut char_px_y: u32 = 0;

    let mut img = RgbImage::new(width, height);

    let mut old_y: u32 = 0;

    for (x, y, _px) in buf.enumerate_pixels() {
        if char_px_x == scaled_font_width {
            char_px_x = 0;
        }

        if old_y != y {
            char_px_x = 0;
            old_y = y;
            char_px_y += 1;

            if char_px_y == scaled_font_height {
                char_px_y = 0;
            }
        }

        let (char_x, char_y) = get_char_x_y(scaled_font_height, scaled_font_width, x, y);

        let c = &char_px[char_x + (char_y * nb_columns as usize)];

        let lightness = c.get_ligthness();

        let char = FONT8X8[gradient_to_char_idx(lightness)];

        let char_px_x_descaled = (char_px_x / args.scale as u32) as u8;
        let char_px_y_descaled = (char_px_y / args.scale as u32) as u8;

        let mask: u8 = 1 << char_px_x_descaled;

        let result: u8 = char[char_px_y_descaled as usize] & mask;

        if result > 0 {
            img.put_pixel(x, y, Rgb([c.avg_r(), c.avg_g(), c.avg_b()]));
        } else {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }

        char_px_x += 1;
    }

    img.save(args.output_file_path + ".png").unwrap();
}
