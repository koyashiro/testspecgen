use std::str::FromStr;

use anyhow::{bail, Error};
use structopt::StructOpt;

#[derive(Debug)]
pub enum Format {
    Markdown,
    Excel,
}

impl FromStr for Format {
    type Err = Error;
    fn from_str(s: &str) -> Result<Format, Self::Err> {
        match s.to_lowercase().as_ref() {
            "markdown" => Ok(Format::Markdown),
            "excel" => Ok(Format::Excel),
            _ => bail!("invalid output format"),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(name = "INPUT")]
    pub input: String,

    #[structopt(name = "OUTPUT")]
    pub output: String,

    #[structopt(
        name = "FORMAT",
        long = "format",
        short = "f",
        default_value = "markdown",
        env
    )]
    pub format: Format,

    #[structopt(name = "NO_COLUMN", long = "no-column", default_value = "No.", env)]
    pub no_column: String,

    #[structopt(
        name = "PRIMARY_ITEM_COLUMN",
        long = "primary-item-column",
        default_value = "Primary Item",
        env
    )]
    pub primary_item_column: String,

    #[structopt(
        name = "SECONDARY_ITEM_COLUMN",
        long = "secondary-item-column",
        default_value = "Secondary Item",
        env
    )]
    pub secondary_item_column: String,

    #[structopt(
        name = "TERTIARY_ITEM_COLUMN",
        long = "tertiary-item-column",
        default_value = "Tertiary Item",
        env
    )]
    pub tertiary_item_column: String,

    #[structopt(
        name = "OPERATOR_COLUMN",
        long = "operator-column",
        default_value = "Operator",
        env
    )]
    pub operator_column: String,

    #[structopt(
        name = "RESULT_COLUMN",
        long = "result-column",
        default_value = "Result",
        env
    )]
    pub result_column: String,

    #[structopt(
        name = "OPERATIONS_COLUMN",
        long = "operations-column",
        default_value = "Operations",
        env
    )]
    pub operations_column: String,

    #[structopt(
        name = "CONFIRMATIONS_COLUMN",
        long = "confirmations-column",
        default_value = "Confirmations",
        env
    )]
    pub confirmations_column: String,

    #[structopt(
        name = "REMARKS_COLUMN",
        long = "remarks-column",
        default_value = "Remarks",
        env
    )]
    pub remarks_column: String,

    #[structopt(name = "FONT", long = "font", default_value = "Yu Gothic", env)]
    pub font: String,
}
