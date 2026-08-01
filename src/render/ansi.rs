use std::fmt::Write as _;
use std::io::{self, Write};

use crate::ascii::{AsciiArt, Color};
use crate::render::{Options, Renderer};

const RESET: &str = "\x1b[0m";

/// Characters with 24-bit terminal color escapes, for piping straight to a
/// terminal.
pub struct AnsiRenderer {
    options: Options,
}

impl AnsiRenderer {
    pub fn new(options: Options) -> AnsiRenderer {
        AnsiRenderer { options }
    }
}

impl Renderer for AnsiRenderer {
    fn extension(&self) -> &'static str {
        "ansi"
    }

    fn is_textual(&self) -> bool {
        true
    }

    fn render(&self, art: &AsciiArt, out: &mut dyn Write) -> io::Result<()> {
        let mut line = String::new();

        for row in art.rows_iter() {
            line.clear();

            // Only re-emit an escape when the color actually changes; runs of
            // one color are common, and the escapes dwarf the characters
            // otherwise.
            let mut current: Option<Color> = None;

            for cell in row {
                let color = cell.color_in(self.options.color_mode);

                if current != Some(color) {
                    let _ = write!(line, "\x1b[38;2;{};{};{}m", color.r, color.g, color.b);
                    current = Some(color);
                }

                line.push(cell.glyph.char());
            }

            line.push_str(RESET);
            line.push('\n');

            out.write_all(line.as_bytes())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ascii::ColorMode;
    use crate::render::tests::checkerboard;

    fn render(mode: ColorMode) -> String {
        let art = checkerboard();
        let mut out = Vec::new();

        AnsiRenderer::new(Options {
            color_mode: mode,
            ..Options::default()
        })
        .render(&art, &mut out)
        .unwrap();

        String::from_utf8(out).unwrap()
    }

    #[test]
    fn it_colors_each_run_once_and_resets_per_line() {
        let out = render(ColorMode::Color);

        assert_eq!(
            out,
            format!(
                "\x1b[38;2;255;255;255m@\x1b[38;2;0;0;0m \x1b[0m\n\
                 \x1b[38;2;0;0;0m \x1b[38;2;255;255;255m@\x1b[0m\n"
            )
        );
    }

    #[test]
    fn monochrome_uses_the_lightness_as_a_gray() {
        assert!(render(ColorMode::Monochrome).contains("\x1b[38;2;255;255;255m@"));
    }
}
