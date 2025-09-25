use image::{ImageReader, Rgb, RgbImage};

fn main() {
    let img = ImageReader::open("hot_dog.png")
        .unwrap()
        .decode()
        .unwrap();

    let font = include_bytes!("../firacode_medium.ttf") as &[u8];
    let firacode = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let (g_metric, _g_bitmap) = firacode.rasterize('g', 10.0);

    let char_height_u32: u32 = 10;
    let char_width_u32: u32 = u32::try_from(g_metric.width).unwrap();

    let buf = img.to_luma8();

    let width = img.width();
    let height = img.height();

    let nb_columns = width / char_width_u32;
    let nb_lines = height / char_height_u32;

    println!("{}x{}", nb_columns, nb_lines);

    let mut img = RgbImage::new(width, height);

    for (i, line) in buf
        .chunks(usize::try_from(width).unwrap())
        .into_iter()
        .enumerate()
    {
        for (j, pixel) in line.iter().enumerate() {
            img.put_pixel(
                u32::try_from(j).unwrap(),
                u32::try_from(i).unwrap(),
                Rgb([*pixel, *pixel, *pixel]),
            );
        }
    }

    img.save("hot_dog_ascii.png").unwrap();
}
