//! # Table Component Module
//!
//! This module provides a table component for displaying tabular data
//! in the SurfDesk application.

use super::combine_classes;
use dioxus::prelude::*;

/// Table component properties
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Component class name
    #[props(optional)]
    pub class: Option<String>,
    /// Component ID
    #[props(optional)]
    pub id: Option<String>,
    /// Table headers
    #[props(optional)]
    pub headers: Option<Vec<String>>,
    /// Table rows (as vector of string vectors)
    #[props(optional)]
    pub rows: Option<Vec<Vec<String>>>,
    /// Whether table is striped
    #[props(default)]
    pub striped: bool,
    /// Whether table is bordered
    #[props(default)]
    pub bordered: bool,
    /// Whether table is hoverable
    #[props(default)]
    pub hoverable: bool,
    /// Whether table is compact
    #[props(default)]
    pub compact: bool,
}

/// Table component
#[component]
pub fn Table(props: TableProps) -> Element {
    let mut classes = vec!["table"];

    if props.striped {
        classes.push("table-striped");
    }

    if props.bordered {
        classes.push("table-bordered");
    }

    if props.hoverable {
        classes.push("table-hoverable");
    }

    if props.compact {
        classes.push("table-compact");
    }

    if let Some(ref class) = props.class {
        classes.push(class);
    }

    let class_attr = combine_classes(&classes);

    rsx! {
        div {
            class: "{class_attr}",
            id: props.id,

            if props.headers.is_some() || props.rows.is_some() {
                table { class: "table-inner",
                    // Table header
                    if let Some(headers) = props.headers {
                        thead { class: "table-header",
                            tr { class: "table-header-row",
                                for header in headers {
                                    th { class: "table-header-cell", "{header}" }
                                }
                            }
                        }
                    }

                    // Table body
                    if let Some(rows) = props.rows {
                        tbody { class: "table-body",
                            for (row_index, row) in rows.iter().enumerate() {
                                tr { class: "table-row",
                                    key: "{row_index}",
                                    for cell in row {
                                        td { class: "table-cell", "{cell}" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "table-empty", "No data to display" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_props() {
        let props = TableProps {
            class: Some("custom-table".to_string()),
            id: Some("data-table".to_string()),
            headers: Some(vec!["Name".to_string(), "Value".to_string()]),
            rows: Some(vec![
                vec!["Item 1".to_string(), "100".to_string()],
                vec!["Item 2".to_string(), "200".to_string()],
            ]),
            striped: true,
            bordered: false,
            hoverable: true,
            compact: false,
        };

        assert!(props.striped);
        assert!(!props.bordered);
        assert!(props.hoverable);
        assert!(!props.compact);
        assert_eq!(props.class, Some("custom-table".to_string()));
        assert_eq!(props.id, Some("data-table".to_string()));
    }
}
