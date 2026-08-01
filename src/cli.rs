use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::ascii::{Color, ColorMode};
use crate::error::{Error, Result};
use crate::font::Ramp;
use crate::render::{AnsiRenderer, Options, PngRenderer, Renderer, SvgRenderer, TextRenderer};

/// Simple Image To Ascii: redraw a PNG with the characters of an 8x8 font.
#[derive(Parser, Debug)]
#[command(name = "sita", version, about, long_about = None)]
pub struct Cli {
    /// PNG file to convert.
    pub input: PathBuf,

    /// Where to write the result. Textual formats go to stdout when this is
    /// left out. A path without an extension gets the format's own.
    pub output: Option<PathBuf>,

    /// Output format. Inferred from the output extension when not given, and
    /// png otherwise.
    #[arg(short, long, value_enum)]
    format: Option<Format>,

    /// Source pixels per glyph pixel: how coarsely the image is sampled. Each
    /// character covers `8 * scale` source pixels on a side.
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=256))]
    scale: u32,

    /// Output pixels (or SVG units) per glyph pixel. Defaults to --scale,
    /// which keeps the output the size of the input.
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..=256))]
    output_scale: Option<u32>,

    /// Whether characters keep the color they were sampled from.
    #[arg(short, long, value_enum, default_value_t = ColorArg::Monochrome)]
    color: ColorArg,

    /// Characters to draw with, darkest first.
    #[arg(long, default_value = Ramp::DEFAULT, value_name = "CHARS")]
    ramp: String,

    /// Reverse the ramp, for dark characters on a light background.
    #[arg(long)]
    invert: bool,

    /// Background color, as #rgb or #rrggbb. Ignored by the text formats.
    #[arg(short, long, default_value = "#000000", value_parser = parse_color)]
    background: Color,

    /// Report timings on stderr.
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum Format {
    /// Raster image.
    Png,
    /// Vector image, one shape per run of glyph pixels.
    Svg,
    /// Plain characters.
    Txt,
    /// Characters with 24-bit terminal color escapes.
    Ansi,
}

impl Format {
    fn from_extension(extension: &str) -> Option<Format> {
        match extension.to_ascii_lowercase().as_str() {
            "png" => Some(Format::Png),
            "svg" => Some(Format::Svg),
            "txt" | "text" => Some(Format::Txt),
            "ansi" => Some(Format::Ansi),
            _ => None,
        }
    }

    fn renderer(self, options: Options) -> Box<dyn Renderer> {
        match self {
            Format::Png => Box::new(PngRenderer::new(options)),
            Format::Svg => Box::new(SvgRenderer::new(options)),
            Format::Txt => Box::new(TextRenderer),
            Format::Ansi => Box::new(AnsiRenderer::new(options)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
enum ColorArg {
    /// Shades of gray, from the lightness of each cell.
    Monochrome,
    /// The average color of each cell.
    Color,
}

impl From<ColorArg> for ColorMode {
    fn from(arg: ColorArg) -> ColorMode {
        match arg {
            ColorArg::Monochrome => ColorMode::Monochrome,
            ColorArg::Color => ColorMode::Color,
        }
    }
}

fn parse_color(raw: &str) -> std::result::Result<Color, String> {
    Color::from_hex(raw).ok_or_else(|| format!("`{raw}` is not a #rgb or #rrggbb color"))
}

/// The command line, resolved into what the pipeline actually needs.
pub struct Config {
    pub input: PathBuf,
    /// Where the rendered bytes go; `None` means stdout.
    pub output: Option<PathBuf>,
    pub scale: u32,
    pub ramp: Ramp,
    pub renderer: Box<dyn Renderer>,
    pub verbose: bool,
}

impl Cli {
    /// Fills in the defaults that depend on other arguments: the format from
    /// the output extension, the output extension from the format, and the
    /// output scale from the sampling scale.
    pub fn resolve(self) -> Result<Config> {
        let ramp = Ramp::new(&self.ramp).map_err(|bad| match bad {
            Some(ch) => Error::UnrenderableRampChar(ch),
            None => Error::EmptyRamp,
        })?;

        let ramp = if self.invert { ramp.inverted() } else { ramp };

        let format = self
            .format
            .or_else(|| {
                let extension = self.output.as_ref()?.extension()?;

                Format::from_extension(&extension.to_string_lossy())
            })
            .unwrap_or(Format::Png);

        let renderer = format.renderer(Options {
            scale: self.output_scale.unwrap_or(self.scale),
            color_mode: self.color.into(),
            background: self.background,
        });

        let output = match self.output {
            // `foo` -> `foo.svg`, but leave `foo.out` alone: the extension may
            // be deliberate, and silently writing elsewhere is worse.
            Some(path) if path.extension().is_none() => {
                Some(path.with_extension(renderer.extension()))
            }
            Some(path) => Some(path),
            None if renderer.is_textual() => None,
            None => return Err(Error::OutputRequired(format!("{format:?}").to_lowercase())),
        };

        Ok(Config {
            input: self.input,
            output,
            scale: self.scale,
            ramp,
            renderer,
            verbose: self.verbose,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve(args: &[&str]) -> Result<Config> {
        Cli::try_parse_from(args).unwrap().resolve()
    }

    #[test]
    fn the_format_comes_from_the_output_extension() {
        let config = resolve(&["sita", "in.png", "out.svg"]).unwrap();

        assert_eq!(config.renderer.extension(), "svg");
        assert_eq!(config.output.unwrap(), PathBuf::from("out.svg"));
    }

    #[test]
    fn an_explicit_format_wins_over_the_extension() {
        let config = resolve(&["sita", "in.png", "out.svg", "-f", "png"]).unwrap();

        assert_eq!(config.renderer.extension(), "png");
        // The path is left as the user typed it.
        assert_eq!(config.output.unwrap(), PathBuf::from("out.svg"));
    }

    #[test]
    fn an_extensionless_output_gets_the_formats_extension() {
        let config = resolve(&["sita", "in.png", "out", "-f", "svg"]).unwrap();

        assert_eq!(config.output.unwrap(), PathBuf::from("out.svg"));
    }

    #[test]
    fn textual_formats_default_to_stdout_and_others_require_a_path() {
        assert!(
            resolve(&["sita", "in.png", "-f", "txt"])
                .unwrap()
                .output
                .is_none()
        );
        assert!(
            resolve(&["sita", "in.png", "-f", "ansi"])
                .unwrap()
                .output
                .is_none()
        );
        assert!(resolve(&["sita", "in.png"]).is_err());
    }

    #[test]
    fn the_ramp_is_validated_and_can_be_inverted() {
        let config = resolve(&["sita", "in.png", "out.txt", "--ramp", "ab"]).unwrap();

        assert_eq!(config.ramp.glyph_for(0).char(), 'a');

        let inverted = resolve(&["sita", "in.png", "out.txt", "--ramp", "ab", "--invert"]).unwrap();

        assert_eq!(inverted.ramp.glyph_for(0).char(), 'b');

        assert!(resolve(&["sita", "in.png", "out.txt", "--ramp", "é"]).is_err());
        assert!(resolve(&["sita", "in.png", "out.txt", "--ramp", ""]).is_err());
    }

    #[test]
    fn scale_must_be_positive_and_colors_must_be_hex() {
        assert!(Cli::try_parse_from(["sita", "in.png", "out.png", "-s", "0"]).is_err());
        assert!(Cli::try_parse_from(["sita", "in.png", "out.png", "-b", "nope"]).is_err());
        assert!(Cli::try_parse_from(["sita", "in.png", "out.png", "-b", "#fff"]).is_ok());
    }

    #[test]
    fn cli_definition_is_valid() {
        use clap::CommandFactory;

        Cli::command().debug_assert();
    }
}
