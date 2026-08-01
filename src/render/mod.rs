//! Output backends.
//!
//! A renderer turns an [`AsciiArt`] into bytes. It only ever sees the cell
//! grid, so adding a format means adding a [`Renderer`] here — nothing in the
//! extraction path needs to know about it.

mod ansi;
mod png;
mod svg;
mod text;

use std::io::{self, Write};

pub use ansi::AnsiRenderer;
pub use png::PngRenderer;
pub use svg::SvgRenderer;
pub use text::TextRenderer;

use crate::ascii::{AsciiArt, Color, ColorMode};

/// Settings shared by the renderers that draw glyphs rather than emit text.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Output pixels (or SVG units) per glyph pixel.
    pub scale: u32,
    pub color_mode: ColorMode,
    /// What to paint where no glyph pixel is set.
    pub background: Color,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            scale: 1,
            color_mode: ColorMode::Monochrome,
            background: Color::BLACK,
        }
    }
}

pub trait Renderer {
    /// Extension for this format, without the dot. Used to complete an output
    /// path that does not have one.
    fn extension(&self) -> &'static str;

    /// Whether the output is text meant for a terminal, in which case the CLI
    /// will write it to stdout when no output path is given.
    fn is_textual(&self) -> bool {
        false
    }

    fn render(&self, art: &AsciiArt, out: &mut dyn Write) -> io::Result<()>;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::ascii::Cell;
    use crate::font::Glyph;

    /// A 2x2 grid alternating a white `@` and a black space, which exercises
    /// both a drawn and a blank glyph and two distinct colors.
    pub(crate) fn checkerboard() -> AsciiArt {
        let lit = Cell {
            glyph: Glyph::from_char('@').unwrap(),
            color: Color::gray(255),
            lightness: 255,
        };

        let dark = Cell {
            glyph: Glyph::from_char(' ').unwrap(),
            color: Color::BLACK,
            lightness: 0,
        };

        AsciiArt::new(2, 2, vec![lit, dark, dark, lit])
    }
}
