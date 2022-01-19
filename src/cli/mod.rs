mod opt;

use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};

use structopt::StructOpt;

use crate::generator::{generate_excel, generate_markdown, ColumnOption, GenerateOption};
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

    let option = GenerateOption {
        column_option: &ColumnOption {
            no: &opt.no_column,
            primary_item: &opt.primary_item_column,
            secondary_item: &opt.secondary_item_column,
            tertiary_item: &opt.tertiary_item_column,
            operator: &opt.operator_column,
            result: &opt.result_column,
            operations: &opt.operations_column,
            confirmations: &opt.confirmations_column,
            remarks: &opt.remarks_column,
        },
        font: &opt.font,
    };
    let generated: Vec<u8> = match opt.format {
        Format::Markdown => generate_markdown(&spec)?.into_bytes(),
        Format::Excel => generate_excel(&spec, &option)?,
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
