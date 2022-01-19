mod excel;
mod markdown;

pub use excel::generate_excel;
pub use markdown::generate_markdown;

#[derive(Debug)]
pub struct GenerateOption<'a> {
    pub column_option: &'a ColumnOption<'a>,
    pub font: &'a str,
}

#[derive(Debug)]
pub struct ColumnOption<'a> {
    pub no: &'a str,
    pub primary_item: &'a str,
    pub secondary_item: &'a str,
    pub tertiary_item: &'a str,
    pub operator: &'a str,
    pub result: &'a str,
    pub operations: &'a str,
    pub confirmations: &'a str,
    pub remarks: &'a str,
}
