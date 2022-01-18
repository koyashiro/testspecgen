use std::fmt::Write;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestSpec {
    pub title: String,

    pub categories: Vec<TestCategory>,
}

impl FromStr for TestSpec {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestCategory {
    pub title: String,

    #[serde(default)]
    pub cases: Vec<TestCase>,
}

impl FromStr for TestCategory {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

pub type Operation = String;

pub type Confirmation = String;

pub type Remark = String;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestCase {
    pub title: String,

    #[serde(default)]
    pub operations: Vec<Operation>,

    #[serde(default)]
    pub confirmations: Vec<Confirmation>,

    #[serde(default)]
    pub remarks: Vec<Remark>,
}

impl FromStr for TestCase {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

pub type Markdown = String;

impl TryInto<Markdown> for TestSpec {
    type Error = std::fmt::Error;
    fn try_into(self) -> Result<Markdown, std::fmt::Error> {
        let TestSpec { title, categories } = &self;
        let mut buf = String::new();
        writeln!(&mut buf, "# {title}")?;

        for TestCategory { title, cases } in categories {
            writeln!(&mut buf)?;
            writeln!(&mut buf, "## {title}")?;

            for TestCase {
                title,
                operations,
                confirmations,
                remarks,
            } in cases
            {
                writeln!(&mut buf)?;
                writeln!(&mut buf, "### {title}")?;

                for (i, operation) in operations.iter().enumerate() {
                    if i == 0 {
                        writeln!(&mut buf)?;
                        writeln!(&mut buf, "#### Operations")?;
                        writeln!(&mut buf)?;
                    }
                    let order = i + 1;
                    writeln!(&mut buf, "{order}. {operation}")?;
                }

                for (i, confirmation) in confirmations.iter().enumerate() {
                    if i == 0 {
                        writeln!(&mut buf)?;
                        writeln!(&mut buf, "#### Confirmation")?;
                        writeln!(&mut buf)?;
                    }
                    writeln!(&mut buf, "- [ ] {confirmation}")?;
                }

                for (i, remark) in remarks.iter().enumerate() {
                    if i == 0 {
                        writeln!(&mut buf)?;
                        writeln!(&mut buf, "#### Remarks")?;
                        writeln!(&mut buf)?;
                    }
                    writeln!(&mut buf, "- {remark}")?;
                }
            }
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_return_ok() {
        let cases = [
            (
                "
title: Spec title
categories: []
",
                TestSpec {
                    title: "Spec title".to_string(),
                    categories: vec![],
                },
            ),
            (
                "
title: Spec title
categories:
  - title: Category 1
",
                TestSpec {
                    title: "Spec title".to_string(),
                    categories: vec![TestCategory {
                        title: "Category 1".to_string(),
                        cases: vec![],
                    }],
                },
            ),
            (
                "
title: Spec title
categories:
  - title: Category 1
    cases:
      - title: Case 1
",
                TestSpec {
                    title: "Spec title".to_string(),
                    categories: vec![TestCategory {
                        title: "Category 1".to_string(),
                        cases: vec![TestCase {
                            title: "Case 1".to_string(),
                            operations: vec![],
                            confirmations: vec![],
                            remarks: vec![],
                        }],
                    }],
                },
            ),
            (
                "
title: Spec title
categories:
  - title: Category 1
    cases:
      - title: Case 1
        operations:
          - Operation 1
          - Operation 2
          - Operation 3
        confirmations:
          - Confirmation 1
          - Confirmation 2
          - Confirmation 3
        remarks:
          - Remark 1
          - Remark 2
          - Remark 3
",
                TestSpec {
                    title: "Spec title".to_string(),
                    categories: vec![TestCategory {
                        title: "Category 1".to_string(),
                        cases: vec![TestCase {
                            title: "Case 1".to_string(),
                            operations: vec![
                                "Operation 1".to_string(),
                                "Operation 2".to_string(),
                                "Operation 3".to_string(),
                            ],
                            confirmations: vec![
                                "Confirmation 1".to_string(),
                                "Confirmation 2".to_string(),
                                "Confirmation 3".to_string(),
                            ],
                            remarks: vec![
                                "Remark 1".to_string(),
                                "Remark 2".to_string(),
                                "Remark 3".to_string(),
                            ],
                        }],
                    }],
                },
            ),
        ];

        for (s, expected) in cases {
            let actual: TestSpec = s.parse().unwrap();
            assert_eq!(expected, actual);
        }
    }
}
