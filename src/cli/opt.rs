use std::str::FromStr;

use anyhow::{bail, ensure, Error};
use clap::Parser;
use regex::Regex;

use crate::generator::{ColumnOption, ColumnsOption, GenerateOption};

#[derive(Debug)]
pub enum Input {
    StdIn,
    Path(String),
}

impl<'a> FromStr for Input {
    type Err = Error;
    fn from_str(s: &str) -> Result<Input, Self::Err> {
        match s.to_lowercase().as_str() {
            "-" => Ok(Input::StdIn),
            _ => Ok(Input::Path(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum Output {
    StdOut,
    Path(String),
}

impl FromStr for Output {
    type Err = Error;
    fn from_str(s: &str) -> Result<Output, Self::Err> {
        match s.to_lowercase().as_str() {
            "-" => Ok(Output::StdOut),
            _ => Ok(Output::Path(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum Format {
    Markdown,
    Excel,
}

impl FromStr for Format {
    type Err = Error;
    fn from_str(s: &str) -> Result<Format, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" => Ok(Format::Markdown),
            "excel" => Ok(Format::Excel),
            _ => bail!("invalid output format: {s}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color(u32);

impl Color {
    pub fn into_inner(&self) -> u32 {
        self.0
    }
}

impl FromStr for Color {
    type Err = Error;
    fn from_str(s: &str) -> Result<Color, Self::Err> {
        let regex = match Regex::new("^0x[0-9a-fA-F]{6}$") {
            Ok(r) => r,
            Err(_) => unreachable!(),
        };
        ensure!(regex.is_match(s), "invalid color: {s}");
        let c = match u32::from_str_radix(&s[2..], 16) {
            Ok(c) => c,
            Err(_) => unreachable!(),
        };
        Ok(Color(c))
    }
}

#[derive(Debug, Parser)]
pub struct Opt {
    #[clap(name = "INPUT")]
    pub input: Input,

    #[clap(name = "OUTPUT")]
    pub output: Output,

    #[clap(
        name = "FORMAT",
        long = "format",
        short = 'f',
        default_value = "markdown",
        env
    )]
    pub format: Format,

    #[clap(name = "NO_HEADER", long = "no-header", default_value = "No.", env)]
    pub no_header: String,

    #[clap(name = "NO_WIDTH", long = "no-width", default_value = "8", env)]
    pub no_width: f64,

    #[clap(
        name = "PRIMARY_ITEM_HEADER",
        long = "primary-item-header",
        default_value = "Primary Item",
        env
    )]
    pub primary_item_header: String,

    #[clap(
        name = "PRIMARY_ITEM_WIDTH",
        long = "primary-item-width",
        default_value = "16",
        env
    )]
    pub primary_item_width: f64,

    #[clap(
        name = "SECONDARY_ITEM_HEADER",
        long = "secondary-item-header",
        default_value = "Secondary Item",
        env
    )]
    pub secondary_item_header: String,

    #[clap(
        name = "SECONDARY_ITEM_WIDTH",
        long = "secondary-item-width",
        default_value = "16",
        env
    )]
    pub secondary_item_width: f64,

    #[clap(
        name = "TERTIARY_ITEM_HEADER",
        long = "tertiary-item-header",
        default_value = "Tertiary Item",
        env
    )]
    pub tertiary_item_header: String,

    #[clap(
        name = "TERTIARY_ITEM_WIDTH",
        long = "tertiary-item-width",
        default_value = "16",
        env
    )]
    pub tertiary_item_width: f64,

    #[clap(
        name = "OPERATOR_HEADER",
        long = "operator-header",
        default_value = "Operator",
        env
    )]
    pub operator_header: String,

    #[clap(
        name = "OPERATOR_WIDTH",
        long = "operator-width",
        default_value = "12",
        env
    )]
    pub operator_width: f64,

    #[clap(
        name = "RESULT_HEADER",
        long = "result-header",
        default_value = "Result",
        env
    )]
    pub result_header: String,

    #[clap(name = "RESULT_WIDTH", long = "result-width", default_value = "8", env)]
    pub result_width: f64,

    #[clap(
        name = "OPERATIONS_HEADER",
        long = "operations-header",
        default_value = "Operations",
        env
    )]
    pub operations_header: String,

    #[clap(
        name = "OPERATIONS_ITEM_WIDTH",
        long = "operations-item-width",
        default_value = "60",
        env
    )]
    pub operations_width: f64,

    #[clap(
        name = "CONFIRMATIONS_HEADER",
        long = "confirmations-header",
        default_value = "Confirmations",
        env
    )]
    pub confirmations_header: String,

    #[clap(
        name = "CONFIRMATIONS_ITEM_WIDTH",
        long = "confirmations-item-width",
        default_value = "60",
        env
    )]
    pub confirmations_width: f64,

    #[clap(
        name = "REMARKS_HEADER",
        long = "remarks-header",
        default_value = "Remarks",
        env
    )]
    pub remarks_header: String,

    #[clap(
        name = "REMARKS_ITEM_WIDTH",
        long = "remarks-item-width",
        default_value = "60",
        env
    )]
    pub remarks_width: f64,

    #[clap(
        name = "FONT_FAMILY",
        long = "font-family",
        default_value = "Yu Gothic",
        env
    )]
    pub font_family: String,

    #[clap(
        name = "HEADER_FONT_COLOR",
        long = "header-font-color",
        default_value = "0xffffff",
        env
    )]
    pub header_font_color: Color,

    #[clap(
        name = "HEADER_BG_COLOR",
        long = "header-bg-color",
        default_value = "0x5b9bd5",
        env
    )]
    pub header_bg_color: Color,

    #[clap(
        name = "BODY_FONT_COLOR",
        long = "body-font-color",
        default_value = "0x000000",
        env
    )]
    pub body_font_color: Color,

    #[clap(
        name = "BODY_BG_COLOR",
        long = "body-bg-color",
        default_value = "0xffffff",
        env
    )]
    pub body_bg_color: Color,

    #[clap(
        name = "BORDER_COLOR",
        long = "border-color",
        default_value = "0x5b9bd5",
        env
    )]
    pub border_color: Color,
}

impl Opt {
    pub fn as_generate_option(&self) -> GenerateOption<'_> {
        GenerateOption {
            column_options: ColumnsOption {
                no_column: ColumnOption {
                    header: &self.no_header,
                    width: self.no_width,
                },
                primary_item_column: ColumnOption {
                    header: &self.primary_item_header,
                    width: self.primary_item_width,
                },
                secondary_item_column: ColumnOption {
                    header: &self.secondary_item_header,
                    width: self.secondary_item_width,
                },
                tertiary_item_column: ColumnOption {
                    header: &self.tertiary_item_header,
                    width: self.tertiary_item_width,
                },
                operator_column: ColumnOption {
                    header: &self.operator_header,
                    width: self.operator_width,
                },
                result_column: ColumnOption {
                    header: &self.result_header,
                    width: self.result_width,
                },
                operations_column: ColumnOption {
                    header: &self.operations_header,
                    width: self.operations_width,
                },
                confirmations_column: ColumnOption {
                    header: &self.confirmations_header,
                    width: self.confirmations_width,
                },
                remarks_column: ColumnOption {
                    header: &self.remarks_header,
                    width: self.remarks_width,
                },
            },
            font_family: &self.font_family,
            header_font_color: self.header_font_color.into_inner(),
            header_bg_color: self.header_bg_color.into_inner(),
            body_font_color: self.body_font_color.into_inner(),
            body_bg_color: self.body_bg_color.into_inner(),
            border_color: self.border_color.into_inner(),
        }
    }
}
