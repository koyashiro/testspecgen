use std::fs::read_to_string;
use std::io::{stdin, Read};

use anyhow::Result;
use structopt::StructOpt;

use super::testspec::parse;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "FILE")]
    file: Option<String>,
}

pub fn execute() -> Result<()> {
    let opt = Opt::from_args();

    let s = match opt.file {
        Some(f) => read_to_string(f)?,
        None => {
            let mut buf = String::new();
            stdin().lock().read_to_string(&mut buf)?;
            buf
        }
    };

    let testspec = parse(&s)?;
    println!("testspec = {:#?}", testspec);

    Ok(())
}
