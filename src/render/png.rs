use std::io::{self, Write};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};

use crate::ascii::AsciiArt;
use crate::font::{GLYPH_HEIGHT, GLYPH_WIDTH};
use crate::render::{Options, Renderer};

/// A raster image, one output pixel per glyph pixel times the scale.
pub struct PngRenderer {
    options: Options,
}

impl PngRenderer {
    pub fn new(options: Options) -> PngRenderer {
        PngRenderer { options }
    }

    /// Paints the art into a flat RGB8 buffer of `width * height * 3` bytes.
    fn rasterize(&self, art: &AsciiArt, width: u32, height: u32) -> Vec<u8> {
        let scale = self.options.scale;
        let stride = (width * 3) as usize;

        let mut buffer = vec![0u8; stride * height as usize];
        let background = self.options.background.channels();

        for px in buffer.chunks_exact_mut(3) {
            px.copy_from_slice(&background);
        }

        for row in 0..art.rows() {
            for column in 0..art.columns() {
                let cell = art.cell(column, row);
                let color = cell.color_in(self.options.color_mode).channels();

                for glyph_y in 0..GLYPH_HEIGHT {
                    for glyph_x in 0..GLYPH_WIDTH {
                        if !cell.glyph.is_set(glyph_x, glyph_y) {
                            continue;
                        }

                        // Top-left output pixel of this glyph pixel.
                        let x0 = (column * GLYPH_WIDTH + glyph_x) * scale;
                        let y0 = (row * GLYPH_HEIGHT + glyph_y) * scale;

                        for y in y0..y0 + scale {
                            let start = y as usize * stride + (x0 * 3) as usize;

                            for px in
                                buffer[start..start + (scale * 3) as usize].chunks_exact_mut(3)
                            {
                                px.copy_from_slice(&color);
                            }
                        }
                    }
                }
            }
        }

        buffer
    }
}

impl Renderer for PngRenderer {
    fn extension(&self) -> &'static str {
        "png"
    }

    fn render(&self, art: &AsciiArt, out: &mut dyn Write) -> io::Result<()> {
        let width = art.width_in_glyph_px() * self.options.scale;
        let height = art.height_in_glyph_px() * self.options.scale;

        let buffer = self.rasterize(art, width, height);

        PngEncoder::new(out)
            .write_image(&buffer, width, height, ExtendedColorType::Rgb8)
            .map_err(io::Error::other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ascii::Color;
    use crate::render::tests::checkerboard;

    fn rasterize(scale: u32, background: Color) -> (u32, Vec<u8>) {
        let art = checkerboard();
        let renderer = PngRenderer::new(Options {
            scale,
            background,
            ..Options::default()
        });

        let width = art.width_in_glyph_px() * scale;
        let height = art.height_in_glyph_px() * scale;

        (width, renderer.rasterize(&art, width, height))
    }

    fn pixel_at(buffer: &[u8], width: u32, x: u32, y: u32) -> [u8; 3] {
        let start = (y * width * 3 + x * 3) as usize;

        buffer[start..start + 3].try_into().unwrap()
    }

    #[test]
    fn unset_pixels_keep_the_background() {
        let (width, buffer) = rasterize(1, Color::new(1, 2, 3));

        // '@' row 0 is 0x3E, so its column 0 is unset, and the cell at (1, 0)
        // is a space throughout.
        assert_eq!(pixel_at(&buffer, width, 0, 0), [1, 2, 3]);
        assert_eq!(pixel_at(&buffer, width, 11, 4), [1, 2, 3]);
    }

    #[test]
    fn set_pixels_take_the_cell_color() {
        let (width, buffer) = rasterize(1, Color::BLACK);

        // '@' row 0 is 0x3E: columns 1..=5 are set, 0 and 6..=7 are not.
        assert_eq!(pixel_at(&buffer, width, 1, 0), [255, 255, 255]);
        assert_eq!(pixel_at(&buffer, width, 5, 0), [255, 255, 255]);
        assert_eq!(pixel_at(&buffer, width, 6, 0), [0, 0, 0]);
    }

    #[test]
    fn scale_grows_every_glyph_pixel_into_a_block() {
        let (width, buffer) = rasterize(3, Color::BLACK);

        assert_eq!(width, 2 * 8 * 3);

        // '@' row 1 is 0x63: glyph pixel (1, 1) is set, (2, 1) is not. At
        // scale 3 they cover x 3..6 and 6..9, both on rows y 3..6.
        for y in 3..6 {
            for x in 3..6 {
                assert_eq!(pixel_at(&buffer, width, x, y), [255, 255, 255]);
            }

            for x in 6..9 {
                assert_eq!(pixel_at(&buffer, width, x, y), [0, 0, 0]);
            }
        }
    }

    #[test]
    fn it_writes_a_decodable_png() {
        let art = checkerboard();
        let mut out = Vec::new();

        PngRenderer::new(Options::default())
            .render(&art, &mut out)
            .unwrap();

        let decoded = image::load_from_memory_with_format(&out, image::ImageFormat::Png).unwrap();

        assert_eq!((decoded.width(), decoded.height()), (16, 16));
    }
}
