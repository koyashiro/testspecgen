use std::fmt::Write;

use crate::testspec::{PrimaryItem, SecondaryItem, TertiaryItem, TestSpec};

pub fn generate_markdown(spec: &TestSpec) -> Result<String, std::fmt::Error> {
    let TestSpec { title, cases } = &spec;
    let mut buf = String::new();
    writeln!(&mut buf, "# {title}")?;

    for PrimaryItem {
        title,
        children: secondaries,
    } in cases
    {
        writeln!(&mut buf)?;
        writeln!(&mut buf, "## {title}")?;

        for SecondaryItem {
            title,
            children: tertiaries,
        } in secondaries
        {
            writeln!(&mut buf)?;
            writeln!(&mut buf, "### {title}")?;

            for TertiaryItem {
                title,
                operations,
                confirmations,
                remarks,
            } in tertiaries
            {
                writeln!(&mut buf)?;
                writeln!(&mut buf, "#### {title}")?;

                for (i, operation) in operations.iter().enumerate() {
                    if i == 0 {
                        writeln!(&mut buf)?;
                        writeln!(&mut buf, "##### Operations")?;
                        writeln!(&mut buf)?;
                    }
                    let order = i + 1;
                    writeln!(&mut buf, "{order}. {operation}")?;
                }

                for (i, confirmation) in confirmations.iter().enumerate() {
                    if i == 0 {
                        writeln!(&mut buf)?;
                        writeln!(&mut buf, "##### Confirmation")?;
                        writeln!(&mut buf)?;
                    }
                    writeln!(&mut buf, "- [ ] {confirmation}")?;
                }

                for (i, remark) in remarks.iter().enumerate() {
                    if i == 0 {
                        writeln!(&mut buf)?;
                        writeln!(&mut buf, "##### Remarks")?;
                        writeln!(&mut buf)?;
                    }
                    writeln!(&mut buf, "- {remark}")?;
                }
            }
        }
    }

    Ok(buf)
}
