use crate::testspec::TestSpec;

use mktemp::Temp;
use xlsxwriter::{FormatAlignment, FormatBorder, FormatColor, Workbook, Worksheet, XlsxError};

use super::GenerateOption;

pub fn generate_excel(spec: &TestSpec, option: &GenerateOption) -> anyhow::Result<Vec<u8>> {
    let temp_file = Temp::new_file()?;
    let filename = match temp_file.to_str() {
        Some(s) => s,
        None => unreachable!(),
    };

    let book = Workbook::new(filename);
    let mut sheet = book.add_worksheet(Some(&spec.title))?;

    let border_color = FormatColor::Custom(0x5b9bd5);

    setup_columns(&mut sheet)?;
    setup_header(&book, &mut sheet, border_color, option)?;
    setup_body(&book, &mut sheet, &spec, border_color, option)?;

    book.close()?;

    let bytes = std::fs::read(filename)?;

    Ok(bytes)
}

fn setup_columns(sheet: &mut Worksheet) -> Result<(), XlsxError> {
    sheet.set_column(0, 0, 8f64, None)?;
    sheet.set_column(1, 1, 16f64, None)?;
    sheet.set_column(2, 2, 16f64, None)?;
    sheet.set_column(3, 3, 16f64, None)?;
    sheet.set_column(4, 4, 12f64, None)?;
    sheet.set_column(5, 5, 8f64, None)?;
    sheet.set_column(6, 6, 60f64, None)?;
    sheet.set_column(7, 7, 60f64, None)?;
    sheet.set_column(8, 8, 60f64, None)?;

    Ok(())
}

fn setup_header(
    book: &Workbook,
    sheet: &mut Worksheet,
    border_color: FormatColor,
    option: &GenerateOption,
) -> Result<(), XlsxError> {
    let header_format = book
        .add_format()
        .set_font_name(option.font)
        .set_border(FormatBorder::Medium)
        .set_border_color(border_color)
        .set_text_wrap()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_font_color(FormatColor::White)
        .set_bold()
        .set_bg_color(border_color);

    sheet.write_string(0, 0, option.column_option.no, Some(&header_format))?;
    sheet.write_string(
        0,
        1,
        option.column_option.primary_item,
        Some(&header_format),
    )?;
    sheet.write_string(
        0,
        2,
        option.column_option.secondary_item,
        Some(&header_format),
    )?;
    sheet.write_string(
        0,
        3,
        option.column_option.tertiary_item,
        Some(&header_format),
    )?;
    sheet.write_string(0, 4, option.column_option.operator, Some(&header_format))?;
    sheet.write_string(0, 5, option.column_option.result, Some(&header_format))?;
    sheet.write_string(0, 6, option.column_option.operations, Some(&header_format))?;
    sheet.write_string(
        0,
        7,
        option.column_option.confirmations,
        Some(&header_format),
    )?;
    sheet.write_string(0, 8, option.column_option.remarks, Some(&header_format))?;

    Ok(())
}

fn setup_body(
    book: &Workbook,
    sheet: &mut Worksheet,
    spec: &TestSpec,
    border_color: FormatColor,
    option: &GenerateOption,
) -> Result<(), XlsxError> {
    let center_align_format = book
        .add_format()
        .set_font_name(option.font)
        .set_text_wrap()
        .set_border(FormatBorder::Medium)
        .set_border_color(border_color)
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter);

    let left_align_format = book
        .add_format()
        .set_font_name(option.font)
        .set_text_wrap()
        .set_border(FormatBorder::Medium)
        .set_border_color(border_color)
        .set_align(FormatAlignment::Left)
        .set_align(FormatAlignment::VerticalTop);

    let mut row = 0;
    for primary in spec.cases.iter() {
        row += 1;
        let primary_start_row_no = row;

        for secondary in primary.children.iter() {
            let secondary_start_row_no = row;

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
                sheet.write_string(row, 3, &tertiary.title, Some(&center_align_format))?;
                sheet.write_blank(row, 4, Some(&center_align_format))?;
                sheet.write_blank(row, 5, Some(&center_align_format))?;
                sheet.write_string(row, 6, &operations_string, Some(&left_align_format))?;
                sheet.write_string(row, 7, &confirmations_string, Some(&left_align_format))?;
                sheet.write_string(row, 8, &remarks_string, Some(&left_align_format))?;
            }

            let secondary_end_row_no = row;
            if secondary_start_row_no == secondary_end_row_no {
                sheet.write_string(row, 2, &secondary.title, Some(&center_align_format))?;
            } else {
                sheet.merge_range(
                    secondary_start_row_no,
                    2,
                    secondary_end_row_no,
                    2,
                    &secondary.title,
                    Some(&center_align_format),
                )?;
            }
        }

        let primary_end_row_no = row;
        if primary_start_row_no == primary_end_row_no {
            sheet.write_string(row, 1, &primary.title, Some(&center_align_format))?;
        } else {
            sheet.merge_range(
                primary_start_row_no,
                1,
                primary_end_row_no,
                1,
                &primary.title,
                Some(&center_align_format),
            )?;
        }
    }

    Ok(())
}
