use std::fs::read_to_string;
use std::io::{self, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use structopt::StructOpt;

use super::generator;

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

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(
        name = "FORMAT",
        long = "format",
        short = "f",
        default_value = "markdown"
    )]
    format: Format,

    #[structopt(name = "FILE")]
    file: String,
}

pub fn execute() -> Result<()> {
    let opt = Opt::from_args();

    let s = match opt.file.as_str() {
        "-" => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            buf
        }
        _ => read_to_string(&opt.file)?,
    };

    let testspec = s.parse()?;
    println!("testspec = {:#?}", testspec);

    enum Output {
        Text(String),
        Binary(Vec<u8>),
    }

    let output: Output = match opt.format {
        Format::Markdown => {
            let markdown = generator::generate_markdown(&testspec);
            Output::Text(markdown)
        }
        Format::Excel => {
            let excel = generator::generate_excel(&testspec);
            Output::Binary(excel)
        }
    };

    match output {
        Output::Text(s) => println!("{}", s),
        Output::Binary(b) => {
            let mut out = io::stdout();
            out.write_all(b.as_ref())?;
            out.flush()?;
        }
    }

    Ok(())
}
