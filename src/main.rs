use image::{ImageReader, Rgb, RgbImage};

fn main() {
    let img = ImageReader::open("hot_dog.png").unwrap().decode().unwrap();

    // mono space font
    let font = include_bytes!("../firacode_medium.ttf") as &[u8];
    let firacode = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let (g_metric, _g_bitmap) = firacode.rasterize('g', 20.0);

    let char_height_u32: u32 = 20;
    let char_width_u32: u32 = u32::try_from(g_metric.width).unwrap();

    let buf = img.to_luma8();

    let width = img.width();
    let height = img.height();

    let nb_columns = width / char_width_u32;
    let nb_lines = height / char_height_u32;

    let mut img = RgbImage::new(width, height);

    let mut char_color: Vec<u32> = vec![0; usize::try_from(nb_columns * nb_lines).unwrap()];

    for (x, y, px) in buf.enumerate_pixels() {
        if x >= char_width_u32*nb_columns || y >= char_height_u32*nb_lines {
            continue;
        }

        let char_x: usize = usize::try_from(x / char_width_u32).unwrap();
        let char_y: usize = usize::try_from(y / char_height_u32).unwrap();

        char_color[char_x + (char_y * usize::try_from(nb_columns).unwrap())] += u32::try_from(px.0[0]).unwrap();
    }

    for (x, y, _px) in buf.enumerate_pixels() {
        if x >= char_width_u32*nb_columns || y >= char_height_u32*nb_lines {
            continue;
        }

        let char_x: usize = usize::try_from(x / char_width_u32).unwrap();
        let char_y: usize = usize::try_from(y / char_height_u32).unwrap();

        let c = char_color[char_x + (char_y * usize::try_from(nb_columns).unwrap())];

        let gradient = u8::try_from(c / (char_height_u32 * char_width_u32)).unwrap();

        img.put_pixel(x, y, Rgb([gradient, gradient, gradient]));
    }

    img.save("hot_dog_ascii.png").unwrap();
}
