use plotters::prelude::*;

/// Plota a série temporal e a linha de regressão em um gráfico.
pub fn plot_series_and_regression(
    x: &[f64],
    y: &[f64],
    a: f64,
    b: f64,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Série Temporal e Regressão Linear", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            x.iter().cloned().fold(f64::INFINITY, f64::min)..x.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            y.iter().cloned().fold(f64::INFINITY, f64::min)..y.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        )?;

    chart.configure_mesh().draw()?;

    // Plota os pontos da série temporal
    chart
        .draw_series(
            x.iter()
                .zip(y.iter())
                .map(|(&xi, &yi)| Circle::new((xi, yi), 5, BLUE.filled())),
        )?
        .label("Dados")
        .legend(|(x, y)| Circle::new((x, y), 5, BLUE.filled()));

    // Plota a linha de regressão
    let regression_line = x
        .iter()
        .map(|&xi| (xi, a * xi + b))
        .collect::<Vec<(f64, f64)>>();
    chart
        .draw_series(LineSeries::new(regression_line, &RED))?
        .label("Regressão Linear")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    // Adiciona estatísticas descritivas ao gráfico
    let mean_value = y.iter().sum::<f64>() / y.len() as f64;
    let min_value = y.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_value = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    chart.draw_series(std::iter::once(Text::new(
        format!(
            "Média: {:.2}\nMínimo: {:.2}\nMáximo: {:.2}",
            mean_value, min_value, max_value
        ),
        (x[0], max_value),
        ("sans-serif", 15).into_font().color(&BLACK),
    )))?;

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    Ok(())
}