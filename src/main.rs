mod chars;

use image::{ImageReader, Rgb, RgbImage};

use crate::chars::{FONT_SIZE_HEIGHT, FONT_SIZE_WIDTH};

fn main() {
    let img = ImageReader::open("hot_dog.png").unwrap().decode().unwrap();

    let _space_char = chars::FONT8X8[0];

    let buf = img.as_rgba8().unwrap();

    let width = img.width();
    let height = img.height();

    let nb_columns: u32 = width / FONT_SIZE_WIDTH.into();
    let nb_lines: u32 = height / FONT_SIZE_HEIGHT.into();

    let mut img = RgbImage::new(width, height);

    let mut char_ligthness: Vec<u32> = vec![0; usize::try_from(nb_columns * nb_lines).unwrap()];

    for (x, y, px) in buf.enumerate_pixels() {
        if x >= FONT_SIZE_WIDTH.into() * nb_columns || y >= FONT_SIZE_HEIGHT.into() * nb_lines {
            continue;
        }

        let char_x: usize = usize::try_from(x / FONT_SIZE_WIDTH.into()).unwrap();
        let char_y: usize = usize::try_from(y / FONT_SIZE_HEIGHT.into()).unwrap();


        let min: u8 = px.0.iter().min().unwrap_or_else(|| 0);

        char_ligthness[char_x + (char_y * usize::try_from(nb_columns).unwrap())] +=
            u32::try_from(px.0[0]).unwrap();
    }

    for (x, y, _px) in buf.enumerate_pixels() {
        if x >= FONT_SIZE_WIDHT * nb_columns || y >= char_height_u32 * nb_lines {
            continue;
        }

        let char_x: usize = usize::try_from(x / FONT_SIZE_WIDHT).unwrap();
        let char_y: usize = usize::try_from(y / char_height_u32).unwrap();

        let px_x_offset = x - (u32::try_from(char_x).unwrap() * FONT_SIZE_WIDHT);
        let px_y_offset = y - (u32::try_from(char_y).unwrap() * char_height_u32);

        let c = char_ligthness[char_x + (char_y * usize::try_from(nb_columns).unwrap())];

        let gradient = u8::try_from(c / (char_height_u32 * FONT_SIZE_WIDHT)).unwrap();

        let mut char_idx = u8::try_from(chars.len()).unwrap() * (gradient / 255);

        if char_idx == 10 {
            char_idx -= 1;
        }

        let cho = &chars[usize::try_from(char_idx).unwrap()].1;

        if cho.len() > 0 {
            let ch = cho[usize::try_from(px_x_offset + px_y_offset * FONT_SIZE_WIDHT).unwrap()];

            if ch == 0 {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                img.put_pixel(x, y, Rgb([gradient / ch, gradient / ch, gradient / ch]));
            }
        } else {
            img.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }

    img.save("hot_dog_ascii.png").unwrap();
}
