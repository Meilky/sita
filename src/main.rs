mod chars;

use image::{ImageReader, Rgb, RgbImage};

use crate::chars::{FONT_SIZE_HEIGHT, FONT_SIZE_WIDTH};

fn main() {
    let img = ImageReader::open("hot_dog.png").unwrap().decode().unwrap();

    let _space_char = chars::FONT8X8[0];

    let buf = img.as_rgba8().unwrap();

    let width = img.width();
    let height = img.height();

    let nb_columns: u32 = width / FONT_SIZE_WIDTH as u32;
    let nb_lines: u32 = height / FONT_SIZE_HEIGHT as u32;

    let mut img = RgbImage::new(width, height);

    let mut char_ligthness: Vec<u32> = vec![0; (nb_columns * nb_lines) as usize];

    println!("ok");

    for (x, y, px) in buf.enumerate_pixels() {
        if x >= FONT_SIZE_WIDTH as u32 * nb_columns || y >= FONT_SIZE_HEIGHT as u32 * nb_lines {
            continue;
        }

        let char_x: usize = (x / FONT_SIZE_WIDTH as u32) as usize;
        let char_y: usize = (y / FONT_SIZE_HEIGHT as u32) as usize;

        let min: u8 = px.0.iter().take(3).min().unwrap_or(&0).clone();
        let max: u8 = px.0.iter().take(3).max().unwrap_or(&0).clone();
        let a: u8 = px.0[3].clone();

        let lightness: u8 = (((min as u16 + max as u16) / 2) * (a / 255) as u16) as u8;

        char_ligthness[char_x + (char_y * nb_columns as usize)] += lightness as u32;
    }


    println!("ok2");

    for (x, y, _px) in buf.enumerate_pixels() {
        if x >= FONT_SIZE_WIDTH as u32 * nb_columns || y >= FONT_SIZE_HEIGHT as u32 * nb_lines {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
            continue;
        }

        let char_x: usize = usize::try_from(x / FONT_SIZE_WIDTH as u32).unwrap();
        let char_y: usize = usize::try_from(y / FONT_SIZE_HEIGHT as u32).unwrap();

        let c = char_ligthness[char_x + (char_y * nb_columns as usize)];

        let gradient: u8 =
            u8::try_from(c / (FONT_SIZE_WIDTH as u32 * FONT_SIZE_HEIGHT as u32)).unwrap();

        img.put_pixel(x, y, Rgb([gradient, gradient, gradient]));
    }

    img.save("hot_dog_ascii.png").unwrap();
}
