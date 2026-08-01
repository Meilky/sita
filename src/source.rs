//! Loading the pictures we turn into ascii.
//!
//! PNG is the only format we accept for now; everything downstream works off
//! [`Source`], so widening this is a matter of adding decoders here.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{ImageFormat, ImageReader, RgbImage};

use crate::error::{Error, Result};

/// A decoded picture, ready to be sampled.
pub struct Source {
    pixels: RgbImage,
}

impl Source {
    /// Decodes `path` as a PNG. Any alpha channel is dropped by compositing
    /// nothing under it, i.e. transparent pixels read as black.
    pub fn load_png(path: impl AsRef<Path>) -> Result<Source> {
        let path = path.as_ref();

        let file = File::open(path).map_err(|e| Error::io(path, e))?;

        let mut reader = ImageReader::new(BufReader::new(file));
        reader.set_format(ImageFormat::Png);

        let image = reader.decode().map_err(|source| match source {
            image::ImageError::Decoding(_) => Error::NotAPng(path.to_path_buf()),
            source => Error::Decode {
                path: path.to_path_buf(),
                source,
            },
        })?;

        let pixels = image.to_rgb8();

        if pixels.width() == 0 || pixels.height() == 0 {
            return Err(Error::EmptyImage(path.to_path_buf()));
        }

        Ok(Source { pixels })
    }

    /// Wraps already-decoded pixels, bypassing the decoders.
    #[cfg(test)]
    pub(crate) fn from_pixels(pixels: RgbImage) -> Source {
        Source { pixels }
    }

    pub fn width(&self) -> u32 {
        self.pixels.width()
    }

    pub fn height(&self) -> u32 {
        self.pixels.height()
    }

    pub fn pixels(&self) -> &RgbImage {
        &self.pixels
    }
}
