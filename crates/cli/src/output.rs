//! Output formatting utilities

use colored::*;
use serde_json;

#[allow(dead_code)]
pub struct OutputFormatter;

impl OutputFormatter {
    #[allow(dead_code)]
    pub fn format_diff_text(changes: &[String]) -> String {
        let mut output = String::new();

        for change in changes {
            if change.starts_with('+') {
                output.push_str(&format!("{}\n", change.green()));
            } else if change.starts_with('-') {
                output.push_str(&format!("{}\n", change.red()));
            } else {
                output.push_str(&format!("{}\n", change));
            }
        }

        output
    }

    #[allow(dead_code)]
    pub fn format_diff_json(changes: &[String]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(changes)
    }

    #[allow(dead_code)]
    pub fn format_diff_html(changes: &[String]) -> String {
        let mut html = String::from("<div class=\"diff\">\n");

        for change in changes {
            if change.starts_with('+') {
                html.push_str(&format!("  <div class=\"addition\">{}</div>\n", change));
            } else if change.starts_with('-') {
                html.push_str(&format!("  <div class=\"deletion\">{}</div>\n", change));
            } else {
                html.push_str(&format!("  <div class=\"context\">{}</div>\n", change));
            }
        }

        html.push_str("</div>");
        html
    }
}
