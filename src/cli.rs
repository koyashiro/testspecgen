use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use structopt::StructOpt;

use crate::generator::{generate_excel, generate_markdown, ColumnOption, GenerateOption};
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

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,

    #[structopt(name = "OUTPUT")]
    output: String,

    #[structopt(
        name = "FORMAT",
        long = "format",
        short = "f",
        default_value = "markdown",
        env
    )]
    format: Format,

    #[structopt(name = "NO_COLUMN", long = "no-column", default_value = "No.", env)]
    no_column: String,

    #[structopt(
        name = "PRIMARY_ITEM_COLUMN",
        long = "primary-item-column",
        default_value = "Primary Item",
        env
    )]
    primary_item_column: String,

    #[structopt(
        name = "SECONDARY_ITEM_COLUMN",
        long = "secondary-item-column",
        default_value = "Secondary Item",
        env
    )]
    secondary_item_column: String,

    #[structopt(
        name = "TERTIARY_ITEM_COLUMN",
        long = "tertiary-item-column",
        default_value = "Tertiary Item",
        env
    )]
    tertiary_item_column: String,

    #[structopt(
        name = "OPERATOR_COLUMN",
        long = "operator-column",
        default_value = "Operator",
        env
    )]
    operator_column: String,

    #[structopt(
        name = "RESULT_COLUMN",
        long = "result-column",
        default_value = "Result",
        env
    )]
    result_column: String,

    #[structopt(
        name = "OPERATIONS_COLUMN",
        long = "operations-column",
        default_value = "Operations",
        env
    )]
    operations_column: String,

    #[structopt(
        name = "CONFIRMATIONS_COLUMN",
        long = "confirmations-column",
        default_value = "Confirmations",
        env
    )]
    confirmations_column: String,

    #[structopt(
        name = "REMARKS_COLUMN",
        long = "remarks-column",
        default_value = "Remarks",
        env
    )]
    remarks_column: String,

    #[structopt(name = "FONT", long = "font", default_value = "Yu Gothic", env)]
    font: String,
}

pub fn execute() -> Result<()> {
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
