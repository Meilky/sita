use fontdue::Metrics;
use image::{ImageReader, Rgb, RgbImage};

fn main() {
    let img = ImageReader::open("hot_dog.png").unwrap().decode().unwrap();

    // mono space font
    let font = include_bytes!("../firacode_medium.ttf") as &[u8];
    let firacode = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let char_height_f32: f32 = 20.0;
    let char_height_u32: u32 = 20;

    // just for the width
    let (g_metric, _g_bitmap) = firacode.rasterize('g', 20.0);
    let char_width_u32: u32 = u32::try_from(g_metric.width).unwrap();

    let chars: Vec<(Metrics, Vec<u8>)> = vec![
        firacode.rasterize(' ', char_height_f32),
        firacode.rasterize('.', char_height_f32),
        firacode.rasterize(':', char_height_f32),
        firacode.rasterize('-', char_height_f32),
        firacode.rasterize('=', char_height_f32),
        firacode.rasterize('+', char_height_f32),
        firacode.rasterize('*', char_height_f32),
        firacode.rasterize('#', char_height_f32),
        firacode.rasterize('%', char_height_f32),
        firacode.rasterize('@', char_height_f32),
    ];

    let buf = img.to_luma8();

    let width = img.width();
    let height = img.height();

    let nb_columns = width / char_width_u32;
    let nb_lines = height / char_height_u32;

    let mut img = RgbImage::new(width, height);

    let mut char_color: Vec<u32> = vec![0; usize::try_from(nb_columns * nb_lines).unwrap()];

    for (x, y, px) in buf.enumerate_pixels() {
        if x >= char_width_u32 * nb_columns || y >= char_height_u32 * nb_lines {
            continue;
        }

        let char_x: usize = usize::try_from(x / char_width_u32).unwrap();
        let char_y: usize = usize::try_from(y / char_height_u32).unwrap();

        char_color[char_x + (char_y * usize::try_from(nb_columns).unwrap())] +=
            u32::try_from(px.0[0]).unwrap();
    }

    for (x, y, _px) in buf.enumerate_pixels() {
        if x >= char_width_u32 * nb_columns || y >= char_height_u32 * nb_lines {
            continue;
        }

        let char_x: usize = usize::try_from(x / char_width_u32).unwrap();
        let char_y: usize = usize::try_from(y / char_height_u32).unwrap();

        let px_x_offset = x - (u32::try_from(char_x).unwrap() * char_width_u32);
        let px_y_offset = y - (u32::try_from(char_y).unwrap() * char_height_u32);

        let c = char_color[char_x + (char_y * usize::try_from(nb_columns).unwrap())];

        let gradient = u8::try_from(c / (char_height_u32 * char_width_u32)).unwrap();

        let mut char_idx = u8::try_from(chars.len()).unwrap() * (gradient / 255);

        if char_idx == 10 {
            char_idx -= 1;
        }

        let cho = &chars[usize::try_from(char_idx).unwrap()].1;

        if cho.len() > 0 {
            let ch = cho[usize::try_from(px_x_offset + px_y_offset * char_width_u32).unwrap()];

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
