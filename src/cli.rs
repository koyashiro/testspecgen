use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use structopt::StructOpt;

use crate::generator::{generate_excel, generate_markdown};
use crate::testspec::TestSpec;

#[derive(Debug)]
enum Format {
    Markdown,
    Excel,
}

impl FromStr for Format {
    type Err = Error;
    fn from_str(s: &str) -> Result<Format, Error> {
        match s.to_lowercase().as_ref() {
            "markdown" => Ok(Format::Markdown),
            "excel" => Ok(Format::Excel),
            _ => bail!("invalid output format"),
        }
    }
}

#[derive(Debug)]
enum Input {
    PathBuf(PathBuf),
    StdIn,
}

impl FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Input, Error> {
        match s.to_lowercase().as_ref() {
            "-" => Ok(Input::StdIn),
            _ => Ok(Input::PathBuf(PathBuf::from(s))),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(
        name = "FORMAT",
        long = "format",
        short = "f",
        default_value = "markdown"
    )]
    format: Format,

    #[structopt(name = "OUTPUT_FILE", long = "output", short = "o")]
    output: Option<PathBuf>,

    #[structopt(name = "INPUT_FILE")]
    input: Input,
}

pub fn execute() -> Result<()> {
    let opt = Opt::from_args();

    let input = match opt.input {
        Input::StdIn => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            buf
        }
        Input::PathBuf(p) => read_to_string(&p)?,
    };

    let spec: TestSpec = input.parse()?;

    let generated: Vec<u8> = match opt.format {
        Format::Markdown => generate_markdown(&spec)?.into_bytes(),
        Format::Excel => generate_excel(&spec)?,
    };

    match &opt.output {
        Some(p) => {
            let mut f = File::create(p)?;
            f.write_all(generated.as_ref())?;
        }
        None => {
            let mut out = io::stdout();
            out.write_all(generated.as_ref())?;
        }
    };

    Ok(())
}
