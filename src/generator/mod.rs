mod excel;
mod markdown;

use std::array::IntoIter;

pub use excel::generate_excel;
pub use markdown::generate_markdown;

#[derive(Debug)]
pub struct GenerateOption<'a> {
    pub column_options: ColumnsOption<'a>,
    pub font_family: &'a str,
    pub header_font_color: u32,
    pub header_bg_color: u32,
    pub body_font_color: u32,
    pub body_bg_color: u32,
    pub border_color: u32,
}

impl Default for GenerateOption<'_> {
    fn default() -> Self {
        GenerateOption {
            column_options: ColumnsOption::default(),
            font_family: "Yu Gothic",
            header_font_color: 0xffffff,
            header_bg_color: 0x5b9bd5,
            body_font_color: 0x000000,
            body_bg_color: 0xffffff,
            border_color: 0x5b9bd5,
        }
    }
}

#[derive(Debug)]
pub struct ColumnsOption<'a> {
    pub no_column: ColumnOption<'a>,
    pub primary_item_column: ColumnOption<'a>,
    pub secondary_item_column: ColumnOption<'a>,
    pub tertiary_item_column: ColumnOption<'a>,
    pub operator_column: ColumnOption<'a>,
    pub result_column: ColumnOption<'a>,
    pub operations_column: ColumnOption<'a>,
    pub confirmations_column: ColumnOption<'a>,
    pub remarks_column: ColumnOption<'a>,
}

impl<'a> IntoIterator for &'a ColumnsOption<'_> {
    type Item = &'a ColumnOption<'a>;
    type IntoIter = IntoIter<Self::Item, 9>;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        [
            &self.no_column,
            &self.primary_item_column,
            &self.secondary_item_column,
            &self.tertiary_item_column,
            &self.operator_column,
            &self.result_column,
            &self.operations_column,
            &self.confirmations_column,
            &self.remarks_column,
        ]
        .into_iter()
    }
}

impl Default for ColumnsOption<'_> {
    fn default() -> Self {
        ColumnsOption {
            no_column: ColumnOption {
                header: "No.",
                width: 8f64,
            },
            primary_item_column: ColumnOption {
                header: "Primary Item",
                width: 16f64,
            },
            secondary_item_column: ColumnOption {
                header: "Secondary Item",
                width: 16f64,
            },
            tertiary_item_column: ColumnOption {
                header: "Tertiary Item",
                width: 16f64,
            },
            operator_column: ColumnOption {
                header: "Operator",
                width: 12f64,
            },
            result_column: ColumnOption {
                header: "Result",
                width: 8f64,
            },
            operations_column: ColumnOption {
                header: "Operations",
                width: 60f64,
            },
            confirmations_column: ColumnOption {
                header: "Confirmations",
                width: 60f64,
            },
            remarks_column: ColumnOption {
                header: "Remarks",
                width: 60f64,
            },
        }
    }
}

#[derive(Debug)]
pub struct ColumnOption<'a> {
    pub header: &'a str,
    pub width: f64,
}
