mod opt;

use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};

use structopt::StructOpt;

use crate::generator::{generate_excel, generate_markdown};
use crate::testspec::TestSpec;

use self::opt::{Format, Input, Opt, Output};

pub fn execute() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let input = match &opt.input {
        Input::StdIn => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            buf
        }
        Input::Path(s) => read_to_string(&s)?,
    };

    let spec: TestSpec = input.parse()?;

    let generate_option = opt.as_generate_option();
    let generated = match opt.format {
        Format::Markdown => generate_markdown(&spec)?.into_bytes(),
        Format::Excel => generate_excel(&spec, &generate_option)?,
    };

    match &opt.output {
        Output::StdOut => {
            let mut out = io::stdout();
            out.write_all(generated.as_ref())?;
        }
        Output::Path(s) => {
            let mut f = File::create(&s)?;
            f.write_all(generated.as_ref())?;
        }
    };

    Ok(())
}
