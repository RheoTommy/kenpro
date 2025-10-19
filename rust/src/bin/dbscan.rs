use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;

use rust::algo::Algo;
use rust::io::{read_points_csv, write_clustered_csv};
use rust::query::RTreeQueryEngine;
use rust::types::Point;

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

fn main() -> Result<()> {
    let Args {
        input,
        output,
        min_points,
        eps,
    } = Args::parse();

    let points = read_points_csv(&input)?;

    // Build a set of references into `points` so the algorithm can refer to them.
    let point_refs: HashSet<&Point> = points.iter().collect();

    let mut engine = RTreeQueryEngine::new();
    let algo = Algo::new(&mut engine, &point_refs, eps, min_points);
    let classes = algo.dbscan();

    write_clustered_csv(&output, &points, &classes)?;
    Ok(())
}
