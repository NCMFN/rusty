use plotters::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("bench_results/plots")?;
    plot_latency()?;
    println!("Plots -> bench_results/plots/*.png");
    Ok(())
}

fn plot_latency() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("bench_results/plots/latency_vs_dim.png", (800, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Mean Latency vs n (1ms SLA)", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(100f64..1100f64, 0.01f64..20f64.log10())?;
    chart
        .configure_mesh()
        .y_desc("ms (log)")
        .x_desc("n")
        .draw()?;
    // data from your run
    let hybrid = vec![
        (128., 0.08),
        (256., 0.18),
        (512., 0.52),
        (768., 0.78),
        (1024., 0.92),
    ];
    let lattice = vec![
        (128., 0.45),
        (256., 1.6),
        (512., 5.8),
        (768., 11.2),
        (1024., 18.5),
    ];
    let hash = vec![
        (128., 0.04),
        (256., 0.04),
        (512., 0.05),
        (768., 0.05),
        (1024., 0.05),
    ];
    chart
        .draw_series(LineSeries::new(hybrid, &GREEN))?
        .label("hybrid")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));
    chart
        .draw_series(LineSeries::new(lattice, &RED))?
        .label("lattice_only")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    chart
        .draw_series(LineSeries::new(hash, &BLUE))?
        .label("hash_only")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
    chart.draw_series(LineSeries::new(
        vec![(128., 1.0), (1024., 1.0)],
        &BLACK.mix(0.5),
    ))?;
    chart.configure_series_labels().draw()?;
    root.present()?;
    Ok(())
}
