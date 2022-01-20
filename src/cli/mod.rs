mod opt;

use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};

use structopt::StructOpt;

use crate::generator::{generate_excel, generate_markdown};
use crate::testspec::TestSpec;

use self::opt::{Format, Opt};

pub fn execute() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let input = match opt.input.as_str() {
        "-" => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            buf
        }
        _ => read_to_string(&opt.input)?,
    };

    let spec: TestSpec = input.parse()?;

    let generate_option = opt.as_generate_option();
    let generated: Vec<u8> = match opt.format {
        Format::Markdown => generate_markdown(&spec)?.into_bytes(),
        Format::Excel => generate_excel(&spec, &generate_option)?,
    };

    match opt.output.as_str() {
        "-" => {
            let mut out = io::stdout();
            out.write_all(generated.as_ref())?;
        }
        _ => {
            let mut f = File::create(&opt.output)?;
            f.write_all(generated.as_ref())?;
        }
    };

    Ok(())
}
