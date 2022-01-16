use serde::{Deserialize, Serialize};
use serde_yaml::from_str;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestSpec {
    pub title: String,

    pub categories: Vec<TestCategory>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestCategory {
    pub title: String,

    #[serde(default)]
    pub cases: Vec<TestCase>,
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

pub fn parse(s: &str) -> serde_yaml::Result<TestSpec> {
    from_str(s)
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
            let actual = parse(&s).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
