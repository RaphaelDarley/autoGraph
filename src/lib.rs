use plotters::prelude::*;
pub fn hl(series: Vec<f64>) -> anyhow::Result<Vec<(f64, f64)>> {
    let mut cumulative = vec![series[0]];
    let mut marginalvcum = vec![1.0];

    for d in &series[1..] {
        let new_cum = cumulative.last().unwrap() + d;
        cumulative.push(new_cum);
        marginalvcum.push(d / new_cum);
    }

    let final_cum = *cumulative.last().unwrap();

    let xy: Vec<(f64, f64)> = cumulative.into_iter().zip(marginalvcum).collect();

    let root = BitMapBackend::new("./hl.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Hubbert Linearization", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..final_cum, 0f64..1f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        xy.iter()
            .map(|(x, y)| Circle::new((*x, *y), 10, GREEN.filled())),
    )?;

    root.present()?;

    todo!();
    Ok(xy)
}
