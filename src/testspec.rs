use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestSpec {
    pub title: String,

    #[serde(default)]
    pub cases: Vec<PrimaryItem>,
}

impl FromStr for TestSpec {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrimaryItem {
    pub title: String,

    #[serde(default)]
    pub children: Vec<SecondaryItem>,
}

impl FromStr for PrimaryItem {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SecondaryItem {
    pub title: String,

    pub children: Vec<TertiaryItem>,
}

impl FromStr for SecondaryItem {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

pub type Operation = String;

pub type Confirmation = String;

pub type Remark = String;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TertiaryItem {
    pub title: String,

    #[serde(default)]
    pub operations: Vec<Operation>,

    #[serde(default)]
    pub confirmations: Vec<Confirmation>,

    #[serde(default)]
    pub remarks: Vec<Remark>,
}

impl FromStr for TertiaryItem {
    type Err = serde_yaml::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_title_only() {
        let s = "
title: Spec title
";
        let expected = TestSpec {
            title: "Spec title".to_string(),
            cases: vec![],
        };
        let actual: TestSpec = s.parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn single() {
        let s = "
title: Spec title

cases:
  - title: Primary 1
    children:
      - title: Secondary 1-1
        children:
          - title: Tertiary 1-1-1
            operations:
              - Operation 1-1-1-1
            confirmations:
              - Confirmation 1-1-1-1
            remarks:
              - Remark 1-1-1-1
";

        let expected = TestSpec {
            title: "Spec title".to_string(),
            cases: vec![PrimaryItem {
                title: "Primary 1".to_string(),
                children: vec![SecondaryItem {
                    title: "Secondary 1-1".to_string(),
                    children: vec![TertiaryItem {
                        title: "Tertiary 1-1-1".to_string(),
                        operations: vec!["Operation 1-1-1-1".to_string()],
                        confirmations: vec!["Confirmation 1-1-1-1".to_string()],
                        remarks: vec!["Remark 1-1-1-1".to_string()],
                    }],
                }],
            }],
        };
        let actual: TestSpec = s.parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn double() {
        let s = "
title: Spec title

cases:
  - title: Primary 1
    children:
      - title: Secondary 1-1
        children:
          - title: Tertiary 1-1-1
            operations:
              - Operation 1-1-1-1
              - Operation 1-1-1-2
            confirmations:
              - Confirmation 1-1-1-1
              - Confirmation 1-1-1-2
            remarks:
              - Remark 1-1-1-1
              - Remark 1-1-1-2

          - title: Tertiary 1-1-2
            operations:
              - Operation 1-1-2-1
              - Operation 1-1-2-2
            confirmations:
              - Confirmation 1-1-2-1
              - Confirmation 1-1-2-2
            remarks:
              - Remark 1-1-2-1
              - Remark 1-1-2-2

      - title: Secondary 1-2
        children:
          - title: Tertiary 1-2-1
            operations:
              - Operation 1-2-1-1
              - Operation 1-2-1-2
            confirmations:
              - Confirmation 1-2-1-1
              - Confirmation 1-2-1-2
            remarks:
              - Remark 1-2-1-1
              - Remark 1-2-1-2

          - title: Tertiary 1-2-2
            operations:
              - Operation 1-2-2-1
              - Operation 1-2-2-2
            confirmations:
              - Confirmation 1-2-2-1
              - Confirmation 1-2-2-2
            remarks:
              - Remark 1-2-2-1
              - Remark 1-2-2-2

  - title: Primary 2
    children:
      - title: Secondary 2-1
        children:
          - title: Tertiary 2-1-1
            operations:
              - Operation 2-1-1-1
              - Operation 2-1-1-2
            confirmations:
              - Confirmation 2-1-1-1
              - Confirmation 2-1-1-2
            remarks:
              - Remark 2-1-1-1
              - Remark 2-1-1-2

          - title: Tertiary 2-1-2
            operations:
              - Operation 2-1-2-1
              - Operation 2-1-2-2
            confirmations:
              - Confirmation 2-1-2-1
              - Confirmation 2-1-2-2
            remarks:
              - Remark 2-1-2-1
              - Remark 2-1-2-2

      - title: Secondary 2-2
        children:
          - title: Tertiary 2-2-1
            operations:
              - Operation 2-2-1-1
              - Operation 2-2-1-2
            confirmations:
              - Confirmation 2-2-1-1
              - Confirmation 2-2-1-2
            remarks:
              - Remark 2-2-1-1
              - Remark 2-2-1-2

          - title: Tertiary 2-2-2
            operations:
              - Operation 2-2-2-1
              - Operation 2-2-2-2
            confirmations:
              - Confirmation 2-2-2-1
              - Confirmation 2-2-2-2
            remarks:
              - Remark 2-2-2-1
              - Remark 2-2-2-2
";

        let expected = TestSpec {
            title: "Spec title".to_string(),
            cases: vec![
                PrimaryItem {
                    title: "Primary 1".to_string(),
                    children: vec![
                        SecondaryItem {
                            title: "Secondary 1-1".to_string(),
                            children: vec![
                                TertiaryItem {
                                    title: "Tertiary 1-1-1".to_string(),
                                    operations: vec![
                                        "Operation 1-1-1-1".to_string(),
                                        "Operation 1-1-1-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 1-1-1-1".to_string(),
                                        "Confirmation 1-1-1-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 1-1-1-1".to_string(),
                                        "Remark 1-1-1-2".to_string(),
                                    ],
                                },
                                TertiaryItem {
                                    title: "Tertiary 1-1-2".to_string(),
                                    operations: vec![
                                        "Operation 1-1-2-1".to_string(),
                                        "Operation 1-1-2-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 1-1-2-1".to_string(),
                                        "Confirmation 1-1-2-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 1-1-2-1".to_string(),
                                        "Remark 1-1-2-2".to_string(),
                                    ],
                                },
                            ],
                        },
                        SecondaryItem {
                            title: "Secondary 1-2".to_string(),
                            children: vec![
                                TertiaryItem {
                                    title: "Tertiary 1-2-1".to_string(),
                                    operations: vec![
                                        "Operation 1-2-1-1".to_string(),
                                        "Operation 1-2-1-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 1-2-1-1".to_string(),
                                        "Confirmation 1-2-1-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 1-2-1-1".to_string(),
                                        "Remark 1-2-1-2".to_string(),
                                    ],
                                },
                                TertiaryItem {
                                    title: "Tertiary 1-2-2".to_string(),
                                    operations: vec![
                                        "Operation 1-2-2-1".to_string(),
                                        "Operation 1-2-2-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 1-2-2-1".to_string(),
                                        "Confirmation 1-2-2-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 1-2-2-1".to_string(),
                                        "Remark 1-2-2-2".to_string(),
                                    ],
                                },
                            ],
                        },
                    ],
                },
                PrimaryItem {
                    title: "Primary 2".to_string(),
                    children: vec![
                        SecondaryItem {
                            title: "Secondary 2-1".to_string(),
                            children: vec![
                                TertiaryItem {
                                    title: "Tertiary 2-1-1".to_string(),
                                    operations: vec![
                                        "Operation 2-1-1-1".to_string(),
                                        "Operation 2-1-1-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 2-1-1-1".to_string(),
                                        "Confirmation 2-1-1-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 2-1-1-1".to_string(),
                                        "Remark 2-1-1-2".to_string(),
                                    ],
                                },
                                TertiaryItem {
                                    title: "Tertiary 2-1-2".to_string(),
                                    operations: vec![
                                        "Operation 2-1-2-1".to_string(),
                                        "Operation 2-1-2-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 2-1-2-1".to_string(),
                                        "Confirmation 2-1-2-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 2-1-2-1".to_string(),
                                        "Remark 2-1-2-2".to_string(),
                                    ],
                                },
                            ],
                        },
                        SecondaryItem {
                            title: "Secondary 2-2".to_string(),
                            children: vec![
                                TertiaryItem {
                                    title: "Tertiary 2-2-1".to_string(),
                                    operations: vec![
                                        "Operation 2-2-1-1".to_string(),
                                        "Operation 2-2-1-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 2-2-1-1".to_string(),
                                        "Confirmation 2-2-1-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 2-2-1-1".to_string(),
                                        "Remark 2-2-1-2".to_string(),
                                    ],
                                },
                                TertiaryItem {
                                    title: "Tertiary 2-2-2".to_string(),
                                    operations: vec![
                                        "Operation 2-2-2-1".to_string(),
                                        "Operation 2-2-2-2".to_string(),
                                    ],
                                    confirmations: vec![
                                        "Confirmation 2-2-2-1".to_string(),
                                        "Confirmation 2-2-2-2".to_string(),
                                    ],
                                    remarks: vec![
                                        "Remark 2-2-2-1".to_string(),
                                        "Remark 2-2-2-2".to_string(),
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        };
        let actual: TestSpec = s.parse().unwrap();
        assert_eq!(expected, actual);
    }
}
