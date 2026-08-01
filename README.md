# Simple Image To Ascii

Redraws a PNG with the characters of an 8x8 bitmap font, to a raster image, an
SVG, or straight to your terminal.

```bash
cargo run --release -- <input.png> [output] [options]
```

```bash
# a png the same size as the input
sita photo.png ascii.png

# vector output, four times the size, keeping the source colors
sita photo.png ascii.svg -s 2 --output-scale 8 -c color

# straight to the terminal
sita photo.png -f txt
sita photo.png -f ansi -c color
```

## Options

| Option | What it does |
| --- | --- |
| `-f`, `--format` | `png`, `svg`, `txt` or `ansi`. Inferred from the output extension, else `png`. |
| `-s`, `--scale` | Source pixels per glyph pixel: how coarsely the image is sampled. Each character covers `8 * scale` source pixels on a side. |
| `--output-scale` | Output pixels (or SVG units) per glyph pixel. Defaults to `--scale`, which keeps the output the size of the input. |
| `-c`, `--color` | `monochrome` (default) or `color`. |
| `--ramp` | The characters to draw with, darkest first. Defaults to `" .:-=+*#%@"`. |
| `--invert` | Reverses the ramp, for dark characters on a light background. |
| `-b`, `--background` | Background color, as `#rgb` or `#rrggbb`. |
| `-v`, `--verbose` | Timings on stderr. |

`txt` and `ansi` go to stdout when no output path is given. An output path
without an extension gets the format's own.

PNG is the only input format for now.

## How it fits together

The pipeline is three independent stages, so a new output format never touches
the image code, and a new input format never touches the renderers.

| Module | Role |
| --- | --- |
| `source` | Decoding an input file into pixels. PNG only, for now. |
| `extract` | Sampling those pixels into cells: average color, lightness, and the glyph the lightness picks off the ramp. |
| `ascii` | The `AsciiArt` grid the two halves meet on. Knows nothing about images or output formats. |
| `render` | The `Renderer` trait and its backends: `png`, `svg`, `text`, `ansi`. |
| `font` | The 8x8 font, its glyphs, and the lightness ramp. |
| `cli` | Argument parsing, and the defaults that depend on other arguments. |

### Adding a renderer

Implement `Renderer` in `src/render/`, export it from `src/render/mod.rs`, and
add a variant to `Format` in `src/cli.rs`. A renderer only ever sees the cell
grid:

```rust
pub trait Renderer {
    fn extension(&self) -> &'static str;
    fn is_textual(&self) -> bool;
    fn render(&self, art: &AsciiArt, out: &mut dyn Write) -> io::Result<()>;
}
```

### The SVG output

Each distinct glyph is defined once in `<defs>`, with its set pixels merged
into horizontal runs of `<rect>`s, and stamped per cell with `<use>`. The
document's user units are glyph pixels, so the art stays crisp at any size —
`--output-scale` only sets the default `width`/`height`.

## Credits

[Inspiration video](https://www.youtube.com/watch?v=t8aSqlC_Duo)

[Another Inspiration video](https://www.youtube.com/watch?v=gg40RWiaHRY)

[8x8 Font Inspiration](https://github.com/dhepper/font8x8/tree/master)
