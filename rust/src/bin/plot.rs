use anyhow::{bail, Context, Result};
use clap::Parser;
use plotters::prelude::*;
use std::fs;

#[derive(Debug, Parser)]
#[command(
    name = "plot",
    author,
    version,
    about = "Plot clustered CSV (cid,x1,x2,...) using Plotters"
)]
struct Args {
    /// Input CSV file: cid,x1,x2,... per line
    input: String,
    /// Output image path (PNG), e.g., out.png
    output: String,

    /// X coordinate column index in the point (0-based, excluding cid)
    #[arg(long, default_value_t = 0)]
    x_col: usize,
    /// Y coordinate column index in the point (0-based, excluding cid)
    #[arg(long, default_value_t = 1)]
    y_col: usize,

    /// Image width in pixels
    #[arg(long, default_value_t = 1000)]
    width: u32,
    /// Image height in pixels
    #[arg(long, default_value_t = 800)]
    height: u32,

    /// Point radius in pixels
    #[arg(long, default_value_t = 2)]
    point_size: i32,

    /// Optional plot title
    #[arg(long, default_value = "Clustering Plot")]
    title: String,
}

#[derive(Clone, Copy)]
struct Sample {
    cid: isize,
    x: f64,
    y: f64,
}

fn parse_csv(path: &str, x_col: usize, y_col: usize) -> Result<Vec<Sample>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read '{}': not found or unreadable", path))?;

    let mut out = Vec::new();
    for (lineno, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if cols.len() < 3 {
            bail!(
                "line {}: expected at least 3 columns (cid,x,y,...)",
                lineno + 1
            );
        }
        let cid: isize = cols[0]
            .parse()
            .with_context(|| format!("line {}: invalid cid '{}'", lineno + 1, cols[0]))?;

        let px = 1 + x_col; // offset by cid column
        let py = 1 + y_col;
        if px >= cols.len() || py >= cols.len() {
            bail!(
                "line {}: x_col/y_col out of bounds for {} data columns",
                lineno + 1,
                cols.len() - 1
            );
        }

        let x: f64 = cols[px]
            .parse()
            .with_context(|| format!("line {}: invalid x '{}'", lineno + 1, cols[px]))?;
        let y: f64 = cols[py]
            .parse()
            .with_context(|| format!("line {}: invalid y '{}'", lineno + 1, cols[py]))?;

        out.push(Sample { cid, x, y });
    }
    if out.is_empty() {
        bail!("no samples found in input");
    }
    Ok(out)
}

fn compute_ranges(samples: &[Sample]) -> ((f64, f64), (f64, f64)) {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    for s in samples {
        x_min = x_min.min(s.x);
        x_max = x_max.max(s.x);
        y_min = y_min.min(s.y);
        y_max = y_max.max(s.y);
    }
    // Add a small margin
    let x_span = (x_max - x_min).abs();
    let y_span = (y_max - y_min).abs();
    let mx = if x_span == 0.0 { 1.0 } else { x_span * 0.05 };
    let my = if y_span == 0.0 { 1.0 } else { y_span * 0.05 };
    ((x_min - mx, x_max + mx), (y_min - my, y_max + my))
}

fn color_for(cid: isize) -> ShapeStyle {
    if cid < 0 {
        return BLACK.mix(0.3).filled();
    }
    // Map cluster id to a palette color deterministically.
    let idx = (cid as usize) % Palette99::COLORS.len();
    let c = Palette99::pick(idx).mix(0.9);
    c.filled()
}

fn draw(samples: &[Sample], args: &Args) -> Result<()> {
    let root = BitMapBackend::new(&args.output, (args.width, args.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let ((x_min, x_max), (y_min, y_max)) = compute_ranges(samples);

    let mut chart = ChartBuilder::on(&root)
        .margin(15)
        .caption(args.title.clone(), ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().x_desc("x").y_desc("y").draw()?;

    chart.draw_series(samples.iter().map(|s| {
        let style = color_for(s.cid);
        Circle::new((s.x, s.y), args.point_size, style)
    }))?;

    root.present().context("failed to write image")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let samples = parse_csv(&args.input, args.x_col, args.y_col)?;
    draw(&samples, &args)
}
