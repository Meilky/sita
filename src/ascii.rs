//! The intermediate representation every renderer consumes.
//!
//! Extraction produces an [`AsciiArt`] and stops there: it knows nothing about
//! output formats, and renderers know nothing about images.

use crate::font::{GLYPH_HEIGHT, GLYPH_WIDTH, Glyph};

/// An 8-bit-per-channel RGB color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color::gray(0);

    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub const fn gray(level: u8) -> Color {
        Color::new(level, level, level)
    }

    /// Parses `#rgb`, `#rrggbb`, or the same without the leading `#`.
    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        let digit = |i: usize| u8::from_str_radix(&hex[i..i + 1], 16).ok();
        let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();

        match hex.len() {
            3 => Some(Color::new(
                digit(0)? * 0x11,
                digit(1)? * 0x11,
                digit(2)? * 0x11,
            )),
            6 => Some(Color::new(byte(0)?, byte(2)?, byte(4)?)),
            _ => None,
        }
    }

    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn channels(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

/// Whether cells keep the color they were sampled from, or are flattened to
/// their lightness.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorMode {
    Monochrome,
    Color,
}

/// One character of the output: which glyph to draw, and in what color.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub glyph: Glyph,
    /// Average color of the source region this cell was sampled from.
    pub color: Color,
    /// Lightness of that same region, which is what picked the glyph.
    pub lightness: u8,
}

impl Cell {
    /// The color to draw this cell in under `mode`.
    pub fn color_in(&self, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Color => self.color,
            ColorMode::Monochrome => Color::gray(self.lightness),
        }
    }
}

/// A grid of [`Cell`]s: the picture, as characters.
pub struct AsciiArt {
    columns: u32,
    rows: u32,
    cells: Vec<Cell>,
}

impl AsciiArt {
    pub(crate) fn new(columns: u32, rows: u32, cells: Vec<Cell>) -> AsciiArt {
        debug_assert_eq!(cells.len(), (columns * rows) as usize);

        AsciiArt {
            columns,
            rows,
            cells,
        }
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// Width of the art in glyph pixels, before any render scaling.
    pub fn width_in_glyph_px(&self) -> u32 {
        self.columns * GLYPH_WIDTH
    }

    /// Height of the art in glyph pixels, before any render scaling.
    pub fn height_in_glyph_px(&self) -> u32 {
        self.rows * GLYPH_HEIGHT
    }

    pub fn cell(&self, column: u32, row: u32) -> &Cell {
        &self.cells[(column + row * self.columns) as usize]
    }

    /// Iterates the grid row by row, left to right.
    pub fn rows_iter(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks_exact(self.columns as usize)
    }
}
