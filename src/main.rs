use auto_graph::hl;
use plotters::prelude::*;

fn main() -> anyhow::Result<()> {
    let res = hl(vec![1.0, 2.0, 4.0, 4.2, 3.9]);
    println!("{:?}", res);

    // let root = SVGBackend::new("./test.svg", (1920, 1080)).into_drawing_area();
    // root.fill(&WHITE)?;
    // let mut chart = ChartBuilder::on(&root)
    //     .caption("y=x^2", ("sans-serif", 50).into_font())
    //     .margin(5)
    //     .x_label_area_size(30)
    //     .y_label_area_size(30)
    //     .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
    //
    // chart.configure_mesh().draw()?;
    //
    // chart
    //     .draw_series(LineSeries::new(
    //         (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
    //         &RED,
    //     ))?
    //     .label("y = x^2")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    //
    // chart
    //     .configure_series_labels()
    //     .background_style(&WHITE.mix(0.8))
    //     .border_style(&BLACK)
    //     .draw()?;
    //
    // root.present()?;

    Ok(())
}
