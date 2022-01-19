use crate::testspec::TestSpec;

use mktemp::Temp;
use xlsxwriter::{FormatAlignment, FormatBorder, FormatColor, Workbook, Worksheet, XlsxError};

pub fn generate_excel(spec: &TestSpec) -> anyhow::Result<Vec<u8>> {
    let temp_file = Temp::new_file()?;
    let filename = match temp_file.to_str() {
        Some(s) => s,
        None => unreachable!(),
    };

    let book = Workbook::new(filename);
    let mut sheet = book.add_worksheet(Some(&spec.title))?;

    let border_color = FormatColor::Custom(0x5b9bd5);

    setup_columns(&mut sheet)?;
    setup_header(&book, &mut sheet, border_color)?;
    setup_body(&book, &mut sheet, &spec, border_color)?;

    book.close()?;

    let bytes = std::fs::read(filename)?;

    Ok(bytes)
}

fn setup_columns(sheet: &mut Worksheet) -> Result<(), XlsxError> {
    sheet.set_column(0, 0, 10f64, None)?;
    sheet.set_column(1, 1, 30f64, None)?;
    sheet.set_column(2, 2, 30f64, None)?;
    sheet.set_column(3, 3, 30f64, None)?;
    sheet.set_column(4, 4, 15f64, None)?;
    sheet.set_column(5, 5, 10f64, None)?;
    sheet.set_column(6, 6, 40f64, None)?;
    sheet.set_column(7, 7, 40f64, None)?;
    sheet.set_column(8, 8, 40f64, None)?;

    Ok(())
}

fn setup_header(
    book: &Workbook,
    sheet: &mut Worksheet,
    border_color: FormatColor,
) -> Result<(), XlsxError> {
    let header_format = book
        .add_format()
        .set_font_name("Yu Gothic")
        .set_border(FormatBorder::Medium)
        .set_border_color(border_color)
        .set_text_wrap()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_font_color(FormatColor::White)
        .set_bold()
        .set_bg_color(border_color);

    sheet.write_string(0, 0, "No.", Some(&header_format))?;
    sheet.write_string(0, 1, "Primary Item", Some(&header_format))?;
    sheet.write_string(0, 2, "Secondary Item", Some(&header_format))?;
    sheet.write_string(0, 3, "Tertiary Item", Some(&header_format))?;
    sheet.write_string(0, 4, "Operator", Some(&header_format))?;
    sheet.write_string(0, 5, "Result", Some(&header_format))?;
    sheet.write_string(0, 6, "Operations", Some(&header_format))?;
    sheet.write_string(0, 7, "Confirmations", Some(&header_format))?;
    sheet.write_string(0, 8, "Remarks", Some(&header_format))?;

    Ok(())
}

fn setup_body(
    book: &Workbook,
    sheet: &mut Worksheet,
    spec: &TestSpec,
    border_color: FormatColor,
) -> Result<(), XlsxError> {
    let center_align_format = book
        .add_format()
        .set_font_name("Yu Gothic")
        .set_text_wrap()
        .set_border(FormatBorder::Medium)
        .set_border_color(border_color)
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter);

    let left_align_format = book
        .add_format()
        .set_font_name("Yu Gothic")
        .set_text_wrap()
        .set_border(FormatBorder::Medium)
        .set_border_color(border_color)
        .set_align(FormatAlignment::Left)
        .set_align(FormatAlignment::VerticalCenter);

    let mut row = 1;
    for primary in spec.cases.iter() {
        for secondary in primary.children.iter() {
            for tertiary in secondary.children.iter() {
                let operations_string = tertiary
                    .operations
                    .iter()
                    .enumerate()
                    .map(|(i, o)| format!("{}. {o}", i + 1))
                    .collect::<Vec<_>>()
                    .join("\n");

                let confirmations_string = tertiary
                    .confirmations
                    .iter()
                    .map(|c| format!("- {c}"))
                    .collect::<Vec<_>>()
                    .join("\n");

                let remarks_string = tertiary
                    .remarks
                    .iter()
                    .map(|r| format!("- {r}"))
                    .collect::<Vec<_>>()
                    .join("\n");

                sheet.write_number(row, 0, row as _, Some(&center_align_format))?;
                sheet.write_string(row, 1, &primary.title, Some(&center_align_format))?;
                sheet.write_string(row, 2, &secondary.title, Some(&center_align_format))?;
                sheet.write_string(row, 3, &tertiary.title, Some(&center_align_format))?;
                sheet.write_blank(row, 4, Some(&center_align_format))?;
                sheet.write_blank(row, 5, Some(&center_align_format))?;
                sheet.write_string(row, 6, &operations_string, Some(&left_align_format))?;
                sheet.write_string(row, 7, &confirmations_string, Some(&left_align_format))?;
                sheet.write_string(row, 8, &remarks_string, Some(&left_align_format))?;

                row += 1;
            }
        }
    }

    Ok(())
}
