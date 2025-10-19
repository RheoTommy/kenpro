use anyhow::{Context, Result};
use clap::Parser;
use plotters::prelude::*;
use std::collections::HashSet;
use rust::algo::RegionQuery;
use rust::query::RTreeQueryEngine;
use rust::io::read_points_csv;
use rust::types::Point;

#[derive(Debug, Parser)]
#[command(
    name = "k-dist",
    author,
    version,
    about = "k-distance plot using RTree (real) query engine"
)]
struct Args {
    /// Input CSV of points: x1,x2,... per line (no header)
    input: String,
    /// Output PNG path for the k-distance plot
    output: String,

    /// k for k-distance (k-th nearest neighbor, excluding self)
    #[arg(long, short = 'k', default_value_t = 4)]
    k: usize,

    /// Image width in pixels
    #[arg(long, default_value_t = 1200)]
    width: u32,
    /// Image height in pixels
    #[arg(long, default_value_t = 800)]
    height: u32,

    /// Optional title
    #[arg(long, default_value = "k-distance plot")]
    title: String,
}

fn compute_k_distances(points: &[Point], k: usize) -> Result<Vec<f64>> {
    let mut engine = RTreeQueryEngine::new();
    let refs: HashSet<&Point> = points.iter().collect();
    engine.init(&refs);

    let mut dists = Vec::with_capacity(points.len());
    for p in points.iter() {
        let d = engine.k_dist(p, k);
        dists.push(d);
    }
    Ok(dists)
}

fn draw_plot(values: &[f64], args: &Args) -> Result<()> {
    let root = BitMapBackend::new(&args.output, (args.width, args.height)).into_drawing_area();
    root.fill(&WHITE)?;

    // Sort descending (common for k-dist plots to inspect the knee)
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let n = sorted.len() as i32;
    let y_min = sorted
        .iter()
        .cloned()
        .fold(f64::INFINITY, |acc, v| acc.min(v));
    let y_max = sorted
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |acc, v| acc.max(v));
    let y_span = (y_max - y_min).abs();
    let y_margin = if y_span == 0.0 { 1.0 } else { y_span * 0.05 };

    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .caption(args.title.clone(), ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(0..n, (y_min - y_margin)..(y_max + y_margin))?;

    chart
        .configure_mesh()
        .x_desc("sorted index")
        .y_desc("k-distance")
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..n).map(|i| (i, sorted[i as usize])),
        &BLUE,
    ))?;

    root.present().context("failed to write image")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let points = read_points_csv(&args.input)?;
    if points.len() < 2 {
        anyhow::bail!("at least 2 points are required");
    }
    if args.k == 0 || args.k >= points.len() {
        anyhow::bail!("k must be in 1..=N-1; got k={}, N={}", args.k, points.len());
    }

    let values = compute_k_distances(&points, args.k)?;
    draw_plot(&values, &args)
}
