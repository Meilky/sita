//! Turning pixels into characters.
//!
//! The source is cut into cells of `cell_size` source pixels; each cell's
//! pixels are averaged, and its lightness picks a glyph off the ramp.

mod px;

use crate::ascii::{AsciiArt, Cell, Color};
use crate::font::{GLYPH_HEIGHT, GLYPH_WIDTH, Ramp};
use crate::source::Source;

use px::Px;

/// Samples a [`Source`] into an [`AsciiArt`].
pub struct Extractor {
    /// Source pixels per glyph pixel. A cell covers `8 * scale` source pixels
    /// on each side.
    scale: u32,
    ramp: Ramp,
}

impl Extractor {
    pub fn new(scale: u32, ramp: Ramp) -> Extractor {
        Extractor {
            scale: scale.max(1),
            ramp,
        }
    }

    /// Width of a cell, in source pixels.
    fn cell_width(&self) -> u32 {
        GLYPH_WIDTH * self.scale
    }

    /// Height of a cell, in source pixels.
    fn cell_height(&self) -> u32 {
        GLYPH_HEIGHT * self.scale
    }

    pub fn extract(&self, source: &Source) -> AsciiArt {
        // Round up, so a partial cell at the right or bottom edge still gets
        // a character.
        let columns = source.width().div_ceil(self.cell_width());
        let rows = source.height().div_ceil(self.cell_height());

        let mut sums = vec![Px::new(); (columns * rows) as usize];

        for (x, y, px) in source.pixels().enumerate_pixels() {
            let column = x / self.cell_width();
            let row = y / self.cell_height();

            sums[(column + row * columns) as usize].add(px.0[0], px.0[1], px.0[2]);
        }

        let cells = sums
            .iter()
            .map(|sum| {
                let lightness = sum.lightness();
                let [r, g, b] = sum.average();

                Cell {
                    glyph: self.ramp.glyph_for(lightness),
                    color: Color::new(r, g, b),
                    lightness,
                }
            })
            .collect();

        AsciiArt::new(columns, rows, cells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ascii::ColorMode;

    use image::{Rgb, RgbImage};

    fn art_from(width: u32, height: u32, scale: u32, fill: Rgb<u8>) -> AsciiArt {
        let mut image = RgbImage::new(width, height);

        for px in image.pixels_mut() {
            *px = fill;
        }

        let extractor = Extractor::new(scale, Ramp::new(Ramp::DEFAULT).unwrap());

        extractor.extract(&Source::from_pixels(image))
    }

    #[test]
    fn a_partial_cell_still_gets_a_column() {
        let art = art_from(9, 8, 1, Rgb([255, 255, 255]));

        assert_eq!(art.columns(), 2);
        assert_eq!(art.rows(), 1);
    }

    #[test]
    fn scale_shrinks_the_grid() {
        let art = art_from(32, 32, 2, Rgb([0, 0, 0]));

        assert_eq!((art.columns(), art.rows()), (2, 2));
    }

    #[test]
    fn a_black_image_is_all_spaces_and_a_white_one_is_all_at_signs() {
        assert_eq!(
            art_from(8, 8, 1, Rgb([0, 0, 0])).cell(0, 0).glyph.char(),
            ' '
        );
        assert_eq!(
            art_from(8, 8, 1, Rgb([255, 255, 255]))
                .cell(0, 0)
                .glyph
                .char(),
            '@'
        );
    }

    #[test]
    fn monochrome_flattens_the_sampled_color() {
        let art = art_from(8, 8, 1, Rgb([255, 0, 0]));
        let cell = art.cell(0, 0);

        assert_eq!(cell.color_in(ColorMode::Color), Color::new(255, 0, 0));
        assert_eq!(
            cell.color_in(ColorMode::Monochrome),
            Color::gray(cell.lightness)
        );
    }
}
