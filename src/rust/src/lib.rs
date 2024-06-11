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

/// @export
#[extendr]
fn plot_values(values: Doubles, x: i32, y: i32) -> String {
    use plotters::prelude::*;

    let size = values.len() as i32;
    let max_val = values.iter().map(|x| x.inner()).reduce(f64::max).unwrap();

    let mut buffer = String::new();
    let root = SVGBackend::with_string(&mut buffer, (x as u32, y as u32)).into_drawing_area();

    root.fill(&TRANSPARENT).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(0..size, 0.0..max_val)
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(WHITE)
        .draw()
        .unwrap();

    chart
        .draw_series(AreaSeries::new(
            (0..).zip(values.iter()).map(|(x, y)| (x, y.inner())),
            0.0,
            RGBColor(232, 86, 0),
        ))
        .unwrap();

    root.present().unwrap();

    drop(chart);
    drop(root);

    buffer
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod redgreen;
    fn redgreen;
    fn plot_values;
}
