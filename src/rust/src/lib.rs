use extendr_api::prelude::*;

/// A very simple funcion that takes in two
/// double arrays. One with the value to
/// render and another with the difference.
/// @export
#[extendr]
fn redgreen(value: Doubles, diff: Doubles) -> Vec<String> {
    let size = value.len();
    let mut spans = Vec::with_capacity(size);
    for i in 0..size {
        let (symbol, class) = if diff[i] >= 0.0 {
            ("▲", "text-success")
        } else {
            ("▼", "text-danger")
        };
        spans.push(format!(
            r#"
            <span class="{class}">{symbol} {value}</span>
            "#,
            value = value[i].inner()
        ));
    }
    spans
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod redgreen;
    fn redgreen;
}
