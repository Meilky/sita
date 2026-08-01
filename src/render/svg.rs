use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::{self, Write};

use crate::ascii::AsciiArt;
use crate::font::{GLYPH_HEIGHT, GLYPH_WIDTH, Glyph};
use crate::render::{Options, Renderer};

/// Vector output: each glyph of the 8x8 font is defined once in `<defs>` as a
/// handful of rectangles, then stamped per cell with `<use>`.
///
/// The document's user units are glyph pixels, so the whole thing stays crisp
/// at any size; `scale` only sets the default `width`/`height`.
pub struct SvgRenderer {
    options: Options,
}

impl SvgRenderer {
    pub fn new(options: Options) -> SvgRenderer {
        SvgRenderer { options }
    }

    /// The `id` a glyph is stamped under. Glyph chars are printable ASCII, but
    /// plenty of them (`#`, `%`, `"`) are awkward in an id, so index by code
    /// point.
    fn glyph_id(glyph: Glyph) -> String {
        format!("g{}", glyph.char() as u32)
    }

    /// One `<g>` per distinct glyph, holding its set pixels merged into
    /// horizontal runs. No `fill` here: each `<use>` supplies its own, and it
    /// inherits into the referenced shapes.
    fn write_defs(&self, art: &AsciiArt, out: &mut String) {
        let mut seen: BTreeMap<u32, Glyph> = BTreeMap::new();

        for row in art.rows_iter() {
            for cell in row {
                seen.insert(cell.glyph.char() as u32, cell.glyph);
            }
        }

        out.push_str("  <defs>\n");

        for glyph in seen.values() {
            if is_blank(*glyph) {
                continue;
            }

            let _ = write!(out, "    <g id=\"{}\">", Self::glyph_id(*glyph));

            for y in 0..GLYPH_HEIGHT {
                for (x, length) in glyph.row_runs(y) {
                    let _ = write!(
                        out,
                        "<rect x=\"{x}\" y=\"{y}\" width=\"{length}\" height=\"1\"/>"
                    );
                }
            }

            out.push_str("</g>\n");
        }

        out.push_str("  </defs>\n");
    }

    fn write_cells(&self, art: &AsciiArt, out: &mut String) {
        for (row, cells) in art.rows_iter().enumerate() {
            for (column, cell) in cells.iter().enumerate() {
                if is_blank(cell.glyph) {
                    continue;
                }

                let x = column as u32 * GLYPH_WIDTH;
                let y = row as u32 * GLYPH_HEIGHT;

                let _ = writeln!(
                    out,
                    "  <use href=\"#{}\" x=\"{x}\" y=\"{y}\" fill=\"{}\"/>",
                    Self::glyph_id(cell.glyph),
                    cell.color_in(self.options.color_mode).to_hex()
                );
            }
        }
    }
}

impl Renderer for SvgRenderer {
    fn extension(&self) -> &'static str {
        "svg"
    }

    fn render(&self, art: &AsciiArt, out: &mut dyn Write) -> io::Result<()> {
        let units_wide = art.width_in_glyph_px();
        let units_high = art.height_in_glyph_px();

        let mut doc = String::new();

        let _ = writeln!(
            doc,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" \
             width=\"{}\" height=\"{}\" viewBox=\"0 0 {units_wide} {units_high}\" \
             shape-rendering=\"crispEdges\">",
            units_wide * self.options.scale,
            units_high * self.options.scale,
        );

        doc.push_str("  <title>sita</title>\n");

        let _ = writeln!(
            doc,
            "  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>",
            self.options.background.to_hex()
        );

        self.write_defs(art, &mut doc);
        self.write_cells(art, &mut doc);

        doc.push_str("</svg>\n");

        out.write_all(doc.as_bytes())
    }
}

/// A glyph with no set pixels draws nothing, so it is worth neither a
/// definition nor a `<use>`.
fn is_blank(glyph: Glyph) -> bool {
    (0..GLYPH_HEIGHT).all(|y| glyph.row(y) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ascii::{Color, ColorMode};
    use crate::render::tests::checkerboard;

    fn render(options: Options) -> String {
        let mut out = Vec::new();

        SvgRenderer::new(options)
            .render(&checkerboard(), &mut out)
            .unwrap();

        String::from_utf8(out).unwrap()
    }

    #[test]
    fn the_viewbox_is_in_glyph_pixels_and_the_size_follows_the_scale() {
        let svg = render(Options {
            scale: 4,
            ..Options::default()
        });

        assert!(svg.contains("viewBox=\"0 0 16 16\""), "{svg}");
        assert!(svg.contains("width=\"64\" height=\"64\""), "{svg}");
    }

    #[test]
    fn each_distinct_glyph_is_defined_once_and_stamped_per_cell() {
        let svg = render(Options::default());
        let at_sign = format!("g{}", '@' as u32);

        assert_eq!(svg.matches(&format!("<g id=\"{at_sign}\">")).count(), 1);
        assert_eq!(svg.matches(&format!("href=\"#{at_sign}\"")).count(), 2);
    }

    #[test]
    fn blank_glyphs_are_left_out() {
        let svg = render(Options::default());

        assert!(!svg.contains(&format!("g{}", ' ' as u32)), "{svg}");
    }

    #[test]
    fn set_pixels_become_runs_rather_than_one_rect_each() {
        let svg = render(Options::default());

        // '@' row 0 is 0x3E: columns 1..=5, one rect rather than five.
        assert!(
            svg.contains("<rect x=\"1\" y=\"0\" width=\"5\" height=\"1\"/>"),
            "{svg}"
        );

        // Row 1 is 0x63: two runs, so a gap is never bridged.
        assert!(
            svg.contains("<rect x=\"0\" y=\"1\" width=\"2\" height=\"1\"/>")
                && svg.contains("<rect x=\"5\" y=\"1\" width=\"2\" height=\"1\"/>"),
            "{svg}"
        );
    }

    #[test]
    fn cells_carry_their_own_fill() {
        let colored = render(Options {
            color_mode: ColorMode::Color,
            background: Color::new(0x11, 0x22, 0x33),
            ..Options::default()
        });

        assert!(colored.contains("fill=\"#ffffff\""), "{colored}");
        assert!(
            colored.contains("<rect width=\"100%\" height=\"100%\" fill=\"#112233\"/>"),
            "{colored}"
        );
    }
}
