use crate::types::{Class, Point};
use anyhow::{Context, Result};
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};

/// Read a CSV of pure coordinates (no header), each line: x1,x2,...,xD
/// Returns points as `Vec<Point>` where `Point = Vec<OrderedFloat<f64>>`.
pub fn read_points_csv(path: &str) -> Result<Vec<Point>> {
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

/// Write clustered output: each line is `cid,x1,x2,...`.
pub fn write_clustered_csv(
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

/// Read clustered CSV: each line `cid,x1,x2,...` into `(cid, Vec<f64>)`.
pub fn read_clustered_csv(path: &str) -> Result<Vec<(isize, Vec<f64>)>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read '{}': not found or unreadable", path))?;

    let mut out = Vec::new();
    let mut expected_dim: Option<usize> = None; // number of coordinates per row
    for (lineno, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if cols.len() < 2 {
            anyhow::bail!(
                "line {}: expected at least 2 columns (cid,x1,...)",
                lineno + 1
            );
        }

        let cid: isize = cols[0]
            .parse()
            .with_context(|| format!("line {}: invalid cid '{}'", lineno + 1, cols[0]))?;

        let coords: Vec<f64> = cols[1..]
            .iter()
            .map(|s| s.parse::<f64>())
            .collect::<std::result::Result<_, _>>()
            .with_context(|| format!("line {}: invalid coordinate value", lineno + 1))?;

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

        out.push((cid, coords));
    }

    if out.is_empty() {
        anyhow::bail!("no samples found in input");
    }
    Ok(out)
}
