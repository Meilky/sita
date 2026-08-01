mod ascii;
mod cli;
mod error;
mod extract;
mod font;
mod render;
mod source;

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;

use crate::cli::Cli;
use crate::error::{Error, Result};
use crate::extract::Extractor;
use crate::source::Source;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("sita: {error}");

            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let started = Instant::now();

    let config = Cli::parse().resolve()?;

    let verbose = config.verbose;

    let mut step = {
        let mut last = started;

        move |label: &str| {
            let now = Instant::now();

            if verbose {
                eprintln!("{label}: {:?}", now - last);
            }

            last = now;
        }
    };

    let source = Source::load_png(&config.input)?;

    step("load");

    let art = Extractor::new(config.scale, config.ramp.clone()).extract(&source);

    step("extract");

    let (mut writer, target) = open_output(config.output.as_deref())?;

    config
        .renderer
        .render(&art, &mut writer)
        .and_then(|()| writer.flush())
        .map_err(|e| Error::io(&target, e))?;

    step("render");

    if config.verbose {
        eprintln!(
            "{}x{} -> {} columns x {} rows in {:?}",
            source.width(),
            source.height(),
            art.columns(),
            art.rows(),
            started.elapsed()
        );
    }

    Ok(())
}

/// Opens the render target, along with a name for it to put in error messages.
fn open_output(path: Option<&std::path::Path>) -> Result<(Box<dyn Write>, PathBuf)> {
    match path {
        Some(path) => {
            let file = File::create(path).map_err(|e| Error::io(path, e))?;

            Ok((Box::new(BufWriter::new(file)), path.to_path_buf()))
        }
        None => Ok((
            Box::new(BufWriter::new(io::stdout().lock())),
            PathBuf::from("<stdout>"),
        )),
    }
}
