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

    setup_columns(&mut sheet, option)?;
    setup_header(&book, &mut sheet, option)?;
    setup_body(&book, &mut sheet, &spec, option)?;

    book.close()?;

    let bytes = std::fs::read(filename)?;

    Ok(bytes)
}

fn setup_columns(sheet: &mut Worksheet, option: &GenerateOption) -> Result<(), XlsxError> {
    for (i, o) in option.column_options.into_iter().enumerate() {
        sheet.set_column(i as _, i as _, o.width, None)?;
    }

    Ok(())
}

fn setup_header(
    book: &Workbook,
    sheet: &mut Worksheet,
    option: &GenerateOption,
) -> Result<(), XlsxError> {
    const ROW: u32 = 0;
    let header_format = book
        .add_format()
        .set_font_name(&option.font_family)
        .set_border(FormatBorder::Medium)
        .set_border_color(FormatColor::Custom(option.border_color))
        .set_text_wrap()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_font_color(FormatColor::Custom(option.header_font_color))
        .set_bold()
        .set_bg_color(FormatColor::Custom(option.header_bg_color));

    for (i, o) in option.column_options.into_iter().enumerate() {
        sheet.write_string(ROW, i as _, &o.header, Some(&header_format))?;
    }

    Ok(())
}

fn setup_body(
    book: &Workbook,
    sheet: &mut Worksheet,
    spec: &TestSpec,
    option: &GenerateOption,
) -> Result<(), XlsxError> {
    let center_align_format = book
        .add_format()
        .set_font_name(&option.font_family)
        .set_text_wrap()
        .set_border(FormatBorder::Medium)
        .set_border_color(FormatColor::Custom(option.border_color))
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter);

    let left_align_format = book
        .add_format()
        .set_font_name(&option.font_family)
        .set_text_wrap()
        .set_border(FormatBorder::Medium)
        .set_border_color(FormatColor::Custom(option.border_color))
        .set_align(FormatAlignment::Left)
        .set_align(FormatAlignment::VerticalTop);

    let mut row = 1;
    for primary in spec.cases.iter() {
        let primary_start_row_no = row;

        if primary.children.is_empty() {
            sheet.write_number(row, 0, row as _, Some(&center_align_format))?;
            sheet.write_blank(row, 1, Some(&center_align_format))?;
            sheet.write_blank(row, 2, Some(&center_align_format))?;
            sheet.write_blank(row, 3, Some(&center_align_format))?;
            sheet.write_blank(row, 4, Some(&center_align_format))?;
            sheet.write_blank(row, 5, Some(&center_align_format))?;
            sheet.write_blank(row, 6, Some(&left_align_format))?;
            sheet.write_blank(row, 7, Some(&left_align_format))?;
            sheet.write_blank(row, 8, Some(&left_align_format))?;
            row += 1;
            continue;
        }

        for secondary in primary.children.iter() {
            let secondary_start_row_no = row;

            if secondary.children.is_empty() {
                sheet.write_number(row, 0, row as _, Some(&center_align_format))?;
                sheet.write_string(row, 1, &primary.title, Some(&center_align_format))?;
                sheet.write_blank(row, 2, Some(&center_align_format))?;
                sheet.write_blank(row, 3, Some(&center_align_format))?;
                sheet.write_blank(row, 4, Some(&center_align_format))?;
                sheet.write_blank(row, 5, Some(&center_align_format))?;
                sheet.write_blank(row, 6, Some(&left_align_format))?;
                sheet.write_blank(row, 7, Some(&left_align_format))?;
                sheet.write_blank(row, 8, Some(&left_align_format))?;
                row += 1;
                continue;
            }

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

                row += 1;
            }

            let secondary_end_row_no = row - 1;
            if secondary_start_row_no == secondary_end_row_no {
                sheet.write_string(
                    secondary_start_row_no,
                    2,
                    &secondary.title,
                    Some(&center_align_format),
                )?;
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

        let primary_end_row_no = row - 1;
        if primary_start_row_no == primary_end_row_no {
            sheet.write_string(
                primary_start_row_no,
                1,
                &primary.title,
                Some(&center_align_format),
            )?;
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
