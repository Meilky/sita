mod chars;

use image::{ImageReader, Rgb, RgbImage};
use std::{env, path::Path};

use crate::chars::{FONT_SIZE_HEIGHT, FONT_SIZE_WIDTH, FONT8X8};

fn is_point_out_of_bound(
    scale: u8,
    font_height: u8,
    font_width: u8,
    nb_columns: u32,
    nb_lines: u32,
    x: u32,
    y: u32,
) -> bool {
    let x_out_of_bound = x >= (font_width as u32 * scale as u32) * nb_columns;
    let y_out_of_bound = y >= (font_height as u32 * scale as u32) * nb_lines;

    x_out_of_bound || y_out_of_bound
}

fn get_char_x_y(scale: u8, font_height: u8, font_width: u8, x: u32, y: u32) -> (usize, usize) {
    let char_x: usize = (x / (font_width as u32 * scale as u32)) as usize;
    let char_y: usize = (y / (font_height as u32 * scale as u32)) as usize;

    (char_x, char_y)
}

fn gradient_to_char_idx(gradient: u8) -> usize {
    let chars_idx: [usize; 10] = [0, 14, 26, 13, 29, 11, 10, 3, 5, 32];
    let steps = chars_idx.len();

    let idx = (gradient as usize * steps) / 256;
    chars_idx[idx]
}

struct Args {
    pub input_file_path: String,
    pub output_file_path: String,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("You need to provide a path to the input file!");
    }

    if args.len() == 2 {
        panic!("You need to provide a path to the output file!");
    }

    let input_file_path = args[1].clone();
    let output_file_path = args[2].clone();

    if Path::new(&input_file_path).exists() == false {
        panic!("The input file must exists!");
    }

    Args {
        input_file_path,
        output_file_path,
    }
}

fn main() {
    let scale: u8 = 1;

    let args = parse_args();

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

    let nb_columns: u32 = width / (FONT_SIZE_WIDTH as u32 * scale as u32);
    let nb_lines: u32 = height / (FONT_SIZE_HEIGHT as u32 * scale as u32);

    let mut img = RgbImage::new(width, height);

    let mut char_ligthness: Vec<u32> = vec![0; (nb_columns * nb_lines) as usize];

    for (x, y, px) in buf.enumerate_pixels() {
        if is_point_out_of_bound(
            scale,
            FONT_SIZE_HEIGHT,
            FONT_SIZE_WIDTH,
            nb_columns,
            nb_lines,
            x,
            y,
        ) {
            continue;
        }

        let (char_x, char_y) = get_char_x_y(scale, FONT_SIZE_HEIGHT, FONT_SIZE_WIDTH, x, y);

        let min: u8 = px.0.iter().take(3).min().unwrap_or(&0).clone();
        let max: u8 = px.0.iter().take(3).max().unwrap_or(&0).clone();

        let lightness: u8 = ((min as u16 + max as u16) / 2) as u8;

        char_ligthness[char_x + (char_y * nb_columns as usize)] += lightness as u32;
    }

    for (x, y, _px) in buf.enumerate_pixels() {
        if is_point_out_of_bound(
            scale,
            FONT_SIZE_HEIGHT,
            FONT_SIZE_WIDTH,
            nb_columns,
            nb_lines,
            x,
            y,
        ) {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
            continue;
        }

        let (char_x, char_y) = get_char_x_y(scale, FONT_SIZE_HEIGHT, FONT_SIZE_WIDTH, x, y);

        let c = char_ligthness[char_x + (char_y * nb_columns as usize)];

        let gradient: u8 = u8::try_from(
            c / (FONT_SIZE_WIDTH as u32 * scale as u32 * FONT_SIZE_HEIGHT as u32 * scale as u32),
        )
        .unwrap();

        let char: [u8; 8] = FONT8X8[gradient_to_char_idx(gradient)];

        let char_px_x = x % FONT_SIZE_WIDTH as u32;
        let char_px_y = y % FONT_SIZE_HEIGHT as u32;

        let mask: u8 = 1 << char_px_x as u8;

        let result: u8 = char[char_px_y as usize] & mask;

        let has_px: bool = result > 0;

        if has_px {
            img.put_pixel(x, y, Rgb([gradient, gradient, gradient]));
        } else {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    img.save(args.output_file_path + ".png").unwrap();
}
