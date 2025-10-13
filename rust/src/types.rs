use ordered_float::OrderedFloat;

pub type Point = Vec<OrderedFloat<f64>>;

/// O(d) where d is the dimensionality of the points.
pub fn dist(a: &Point, b: &Point) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let d = x.0 - y.0;
            d * d
        })
        .sum::<f64>()
        .sqrt()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Class {
    Unclassified,
    Classified(usize),
    Noise,
}
