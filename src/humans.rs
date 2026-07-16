pub fn trim_stringfloat(value: String) -> String {
    if let Some(_) = value.find('.') {
        value
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        value
    }
}

pub fn format_float_with_precision(value: f64, precision: usize) -> String {
    let rounded = if value.abs() < 1e-12 {
        0.0
    } else {
        value
    };

    format!("{:.*}", precision, rounded)
        .to_string()
}

pub fn get_max_precision_from_list(values: &[f64]) -> usize {
    let mut max_precision = 0;

    for &value in values {
        let formatted = trim_stringfloat(value.to_string());
        if let Some(pos) = formatted.find('.') {
            let precision = formatted.len() - pos - 1;
            if precision > max_precision {
                max_precision = precision;
            }
        }
    }

    max_precision
}

// Quick function to do everything here
pub fn format_float_listbased(list: &[f64], value: f64) -> String {
    let max_precision = get_max_precision_from_list(list);
    let cleaned_value = format_float_with_precision(value, max_precision);
    trim_stringfloat(cleaned_value)
}

// Or do this instead if you have a list
pub fn format_floatlist(list: &[f64]) -> Vec<String> {
    let max_precision = get_max_precision_from_list(list);
    list.iter()
        .map(|&value| {
            let cleaned_value = format_float_with_precision(value, max_precision);
            trim_stringfloat(cleaned_value)
        })
        .collect()
}
