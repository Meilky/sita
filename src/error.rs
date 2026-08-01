use std::fmt;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The input file could not be read, or the output file could not be
    /// written.
    Io { path: PathBuf, source: io::Error },
    /// The input is not something we can decode as a PNG.
    NotAPng(PathBuf),
    /// The input decoded, but not into anything we can turn into ascii.
    Decode {
        path: PathBuf,
        source: image::ImageError,
    },
    /// The input has no pixels to sample.
    EmptyImage(PathBuf),
    /// The requested ramp contains a character the font cannot render.
    UnrenderableRampChar(char),
    /// The requested ramp is empty.
    EmptyRamp,
    /// A binary format was asked for with nowhere to put it.
    OutputRequired(String),
}

impl Error {
    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Error {
        Error::Io {
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Error::NotAPng(path) => {
                write!(f, "{}: only PNG input is supported for now", path.display())
            }
            Error::Decode { path, source } => {
                write!(f, "{}: could not decode PNG: {source}", path.display())
            }
            Error::EmptyImage(path) => write!(f, "{}: image has no pixels", path.display()),
            Error::UnrenderableRampChar(ch) => write!(
                f,
                "the ramp character {ch:?} is not in the 8x8 font (printable ASCII only)"
            ),
            Error::EmptyRamp => write!(f, "the ramp must contain at least one character"),
            Error::OutputRequired(format) => write!(
                f,
                "the {format} format needs an output path; pass one, or use --format txt to print to stdout"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            Error::Decode { source, .. } => Some(source),
            _ => None,
        }
    }
}
