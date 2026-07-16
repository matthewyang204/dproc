use std::fmt;
use std::process::exit;

pub fn format_float(value: f64) -> String {
    let rounded = if value.abs() < 1e-12 {
        0.0
    } else {
        value
    };

    format!("{:.12}", rounded)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

