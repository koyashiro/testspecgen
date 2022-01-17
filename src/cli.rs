use std::fs::read_to_string;
use std::io::{self, Read, Write};
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use structopt::StructOpt;

use super::generator;

#[derive(Debug)]
enum OutputFormat {
    Markdown,
    Json,
    Yaml,
    Excel,
}

impl FromStr for OutputFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<OutputFormat, Error> {
        match s.to_lowercase().as_ref() {
            "markdown" => Ok(OutputFormat::Markdown),
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "excel" => Ok(OutputFormat::Excel),
            _ => bail!("invalid output format"),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, default_value = "markdown")]
    output_format: OutputFormat,

    #[structopt(name = "FILE")]
    file: Option<String>,
}

pub fn execute() -> Result<()> {
    let opt = Opt::from_args();

    let s = match opt.file {
        Some(f) => read_to_string(f)?,
        None => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            buf
        }
    };

    let testspec = s.parse()?;
    println!("testspec = {:#?}", testspec);

    enum Output {
        Text(String),
        Binary(Vec<u8>),
    }

    let output: Output = match opt.output_format {
        OutputFormat::Markdown => {
            let markdown = generator::generate_markdown(&testspec);
            Output::Text(markdown)
        }
        OutputFormat::Yaml => {
            let yaml = generator::generate_yaml(&testspec);
            Output::Text(yaml)
        }
        OutputFormat::Json => {
            let json = generator::generate_json(&testspec);
            Output::Text(json)
        }
        OutputFormat::Excel => {
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
