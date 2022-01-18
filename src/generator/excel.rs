use crate::testspec::TestSpec;

use mktemp::Temp;
use xlsxwriter::{FormatAlignment, Workbook};

pub fn generate_excel(spec: &TestSpec) -> anyhow::Result<Vec<u8>> {
    let temp_file = Temp::new_file()?;
    let filename = match temp_file.to_str() {
        Some(s) => s,
        None => unreachable!(),
    };

    let book = Workbook::new(filename);
    let mut sheet = book.add_worksheet(Some(&spec.title))?;

    let center_align_format = book
        .add_format()
        .set_align(FormatAlignment::Center)
        .set_align(FormatAlignment::VerticalCenter)
        .set_font_name("Yu Gothic")
        .set_text_wrap();
    let left_align_format = book
        .add_format()
        .set_align(FormatAlignment::Left)
        .set_align(FormatAlignment::VerticalCenter)
        .set_font_name("Yu Gothic")
        .set_text_wrap();

    // Column Width
    sheet.set_column(0, 0, 5f64, Some(&center_align_format))?;
    sheet.set_column(1, 1, 30f64, Some(&center_align_format))?;
    sheet.set_column(2, 2, 30f64, Some(&center_align_format))?;
    sheet.set_column(3, 3, 30f64, Some(&center_align_format))?;
    sheet.set_column(4, 4, 15f64, Some(&center_align_format))?;
    sheet.set_column(5, 5, 10f64, Some(&center_align_format))?;
    sheet.set_column(6, 6, 40f64, Some(&center_align_format))?;
    sheet.set_column(7, 7, 40f64, Some(&center_align_format))?;
    sheet.set_column(8, 8, 40f64, Some(&center_align_format))?;

    // Header
    sheet.write_string(0, 0, "No.", Some(&center_align_format))?;
    sheet.write_string(0, 1, "Primary Item", Some(&center_align_format))?;
    sheet.write_string(0, 2, "Secondary Item", Some(&center_align_format))?;
    sheet.write_string(0, 3, "Tertiary Item", Some(&center_align_format))?;
    sheet.write_string(0, 4, "Operator", Some(&center_align_format))?;
    sheet.write_string(0, 5, "Result", Some(&center_align_format))?;
    sheet.write_string(0, 6, "Operations", Some(&center_align_format))?;
    sheet.write_string(0, 7, "Confirmations", Some(&center_align_format))?;
    sheet.write_string(0, 8, "Remarks", Some(&center_align_format))?;

    // Body
    let mut i = 0u32;
    for primary in spec.cases.iter() {
        for secondary in primary.children.iter() {
            for tertiary in secondary.children.iter() {
                i += 1;

                let operations = tertiary
                    .operations
                    .iter()
                    .enumerate()
                    .map(|(i, o)| {
                        let no = i + 1;
                        format!("{no}. {o}")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let confirmations = tertiary
                    .confirmations
                    .iter()
                    .map(|c| format!("- {c}"))
                    .collect::<Vec<_>>()
                    .join("\n");

                let remarks = tertiary
                    .remarks
                    .iter()
                    .map(|r| format!("- {r}"))
                    .collect::<Vec<_>>()
                    .join("\n");

                sheet.write_number(i, 0, i as _, Some(&center_align_format))?;
                sheet.write_string(i, 1, &primary.title, Some(&center_align_format))?;
                sheet.write_string(i, 2, &secondary.title, Some(&center_align_format))?;
                sheet.write_string(i, 3, &tertiary.title, Some(&center_align_format))?;
                sheet.write_string(i, 6, &operations, Some(&left_align_format))?;
                sheet.write_string(i, 7, &confirmations, Some(&left_align_format))?;
                sheet.write_string(i, 8, &remarks, Some(&left_align_format))?;
            }
        }
    }

    book.close()?;

    let bytes = std::fs::read(filename)?;
    Ok(bytes)
}
