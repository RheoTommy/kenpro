use anyhow::{Context, Result};
use clap::Parser;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufWriter, Write};

use rust::algo::Algo;
use rust::fake_query::FakeQueryEngine;
use rust::types::{Class, Point};

#[derive(Debug, Parser)]
#[command(
    name = "dbscan",
    author,
    version,
    about = "Density-based clustering (DBSCAN)"
)]
struct Args {
    /// Input CSV file with points: x11,x12,...,x1D per line
    input: String,
    /// Output CSV file: cid,x1,x2,...,xD per line
    output: String,
    /// Minimum number of points to form a dense region
    min_points: usize,
    /// Neighborhood radius (epsilon)
    eps: f64,
}

fn parse_csv_points(path: &str) -> Result<Vec<Point>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read '{}': not found or unreadable", path))?;

    let mut points: Vec<Point> = Vec::new();
    let mut expected_dim: Option<usize> = None;

    for (lineno, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let coords: Vec<OrderedFloat<f64>> = line
            .split(',')
            .map(|s| s.trim().parse::<f64>().map(OrderedFloat))
            .collect::<std::result::Result<_, _>>()
            .with_context(|| format!("parse error at line {}", lineno + 1))?;

        if let Some(dim) = expected_dim {
            if coords.len() != dim {
                anyhow::bail!(
                    "dimension mismatch at line {}: expected {}, got {}",
                    lineno + 1,
                    dim,
                    coords.len()
                );
            }
        } else {
            expected_dim = Some(coords.len());
        }

        points.push(coords);
    }

    if points.is_empty() {
        anyhow::bail!("no points found in input");
    }

    Ok(points)
}

fn write_clustered_csv(
    path: &str,
    points: &[Point],
    classes: &HashMap<&Point, Class>,
) -> Result<()> {
    let file = fs::File::create(path).with_context(|| {
        format!(
            "failed to create '{}': insufficient permissions or path invalid",
            path
        )
    })?;
    let mut w = BufWriter::new(file);

    for p in points.iter() {
        let cid = match classes.get(p).copied().unwrap_or(Class::Noise) {
            Class::Classified(id) => id as isize,
            Class::Noise | Class::Unclassified => -1,
        };

        write!(w, "{}", cid)?;
        for x in p.iter() {
            write!(w, ",{}", x.0)?;
        }
        writeln!(w)?;
    }

    w.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let Args {
        input,
        output,
        min_points,
        eps,
    } = Args::parse();

    let points = parse_csv_points(&input)?;

    // Build a set of references into `points` so the algorithm can refer to them.
    let point_refs: HashSet<&Point> = points.iter().collect();

    let mut engine = FakeQueryEngine::new();
    let algo = Algo::new(&mut engine, &point_refs, eps, min_points);
    let classes = algo.dbscan();

    write_clustered_csv(&output, &points, &classes)?;
    Ok(())
}
