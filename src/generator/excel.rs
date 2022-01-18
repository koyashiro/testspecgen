use crate::testspec::TestSpec;

use simple_excel_writer::{row, Column, Row, Workbook};

pub fn generate_excel(spec: &TestSpec) -> Result<Vec<u8>, std::io::Error> {
    let mut book = Workbook::create_in_memory();
    let mut sheet = book.create_sheet(&spec.title);

    // No
    sheet.add_column(Column { width: 10f32 });
    // Primary item
    sheet.add_column(Column { width: 20f32 });
    // Secondary item
    sheet.add_column(Column { width: 30f32 });
    // Tertiary item
    sheet.add_column(Column { width: 50f32 });
    // Operator
    sheet.add_column(Column { width: 10f32 });
    // Test result
    sheet.add_column(Column { width: 10f32 });
    // Operations
    sheet.add_column(Column { width: 40f32 });
    // Confirmations
    sheet.add_column(Column { width: 40f32 });
    // Remarks
    sheet.add_column(Column { width: 30f32 });

    book.write_sheet(&mut sheet, |sw| {
        sw.append_row(row![
            "No.",
            "Primary item",
            "Secondary item",
            "Tertiary item",
            "Operator",
            "Test result",
            "Operations",
            "Confirmations",
            "Remarks"
        ])?;

        let mut i = 0u32;
        for primary in spec.cases.iter() {
            for secondary in primary.children.iter() {
                for tertiary in secondary.children.iter() {
                    let operation = tertiary
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

                    i += 1;
                    sw.append_row(row![
                        i.to_string(),
                        primary.title.as_ref(),
                        secondary.title.as_ref(),
                        tertiary.title.as_ref(),
                        (),
                        (),
                        operation,
                        confirmations,
                        remarks
                    ])?;
                }
            }
        }
        Ok(())
    })?;

    book.close().map(|v| v.expect("book is None"))
}
