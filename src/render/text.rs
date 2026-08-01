use std::io::{self, Write};

use crate::ascii::AsciiArt;
use crate::render::Renderer;

/// Plain characters, one line per cell row. Ignores color entirely.
pub struct TextRenderer;

impl Renderer for TextRenderer {
    fn extension(&self) -> &'static str {
        "txt"
    }

    fn is_textual(&self) -> bool {
        true
    }

    fn render(&self, art: &AsciiArt, out: &mut dyn Write) -> io::Result<()> {
        let mut line = String::with_capacity(art.columns() as usize + 1);

        for row in art.rows_iter() {
            line.clear();
            line.extend(row.iter().map(|cell| cell.glyph.char()));
            line.push('\n');

            out.write_all(line.as_bytes())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::tests::checkerboard;

    #[test]
    fn it_writes_one_line_per_row() {
        let art = checkerboard();
        let mut out = Vec::new();

        TextRenderer.render(&art, &mut out).unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "@ \n @\n");
    }
}
