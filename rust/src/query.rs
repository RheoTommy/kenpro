use crate::algo::RegionQuery;
use crate::types::{dist, Point};
use rstar::primitives::GeomWithData;
use rstar::RTree;
use std::collections::HashSet;

pub struct RTreeQueryEngine<'a> {
    inner: Option<RTreeAnyDim<'a>>,
    dim: usize,
}

impl<'a> RTreeQueryEngine<'a> {
    pub fn new() -> Self {
        Self {
            inner: None,
            dim: 0,
        }
    }
}

enum RTreeAnyDim<'a> {
    D1(RTree<GeomWithData<[f64; 1], &'a Point>>),
    D2(RTree<GeomWithData<[f64; 2], &'a Point>>),
    D3(RTree<GeomWithData<[f64; 3], &'a Point>>),
    D4(RTree<GeomWithData<[f64; 4], &'a Point>>),
    D5(RTree<GeomWithData<[f64; 5], &'a Point>>),
    D6(RTree<GeomWithData<[f64; 6], &'a Point>>),
    D7(RTree<GeomWithData<[f64; 7], &'a Point>>),
    D8(RTree<GeomWithData<[f64; 8], &'a Point>>),
    D9(RTree<GeomWithData<[f64; 9], &'a Point>>),
    D10(RTree<GeomWithData<[f64; 10], &'a Point>>),
    D11(RTree<GeomWithData<[f64; 11], &'a Point>>),
    D12(RTree<GeomWithData<[f64; 12], &'a Point>>),
    D13(RTree<GeomWithData<[f64; 13], &'a Point>>),
    D14(RTree<GeomWithData<[f64; 14], &'a Point>>),
    D15(RTree<GeomWithData<[f64; 15], &'a Point>>),
    D16(RTree<GeomWithData<[f64; 16], &'a Point>>),
}

// Small helper macro to dispatch over the concrete dimensionality at runtime
// and run a block with bindings: `$tree` (the RTree) and const `$N` (usize).
macro_rules! with_dim {
    ($inner:expr, |$tree:ident, $N:ident| $body:block) => {
        match $inner {
            RTreeAnyDim::D1($tree) => {
                const $N: usize = 1;
                $body
            }
            RTreeAnyDim::D2($tree) => {
                const $N: usize = 2;
                $body
            }
            RTreeAnyDim::D3($tree) => {
                const $N: usize = 3;
                $body
            }
            RTreeAnyDim::D4($tree) => {
                const $N: usize = 4;
                $body
            }
            RTreeAnyDim::D5($tree) => {
                const $N: usize = 5;
                $body
            }
            RTreeAnyDim::D6($tree) => {
                const $N: usize = 6;
                $body
            }
            RTreeAnyDim::D7($tree) => {
                const $N: usize = 7;
                $body
            }
            RTreeAnyDim::D8($tree) => {
                const $N: usize = 8;
                $body
            }
            RTreeAnyDim::D9($tree) => {
                const $N: usize = 9;
                $body
            }
            RTreeAnyDim::D10($tree) => {
                const $N: usize = 10;
                $body
            }
            RTreeAnyDim::D11($tree) => {
                const $N: usize = 11;
                $body
            }
            RTreeAnyDim::D12($tree) => {
                const $N: usize = 12;
                $body
            }
            RTreeAnyDim::D13($tree) => {
                const $N: usize = 13;
                $body
            }
            RTreeAnyDim::D14($tree) => {
                const $N: usize = 14;
                $body
            }
            RTreeAnyDim::D15($tree) => {
                const $N: usize = 15;
                $body
            }
            RTreeAnyDim::D16($tree) => {
                const $N: usize = 16;
                $body
            }
        }
    };
}

fn to_array<const N: usize>(p: &Point) -> [f64; N] {
    assert_eq!(
        p.len(),
        N,
        "point dimension mismatch: expected {}, got {}",
        N,
        p.len()
    );
    let mut arr = [0.0_f64; N];
    for i in 0..N {
        arr[i] = p[i].0;
    }
    arr
}

fn build_tree<'a, const N: usize>(
    points: &'a HashSet<&'a Point>,
) -> RTree<GeomWithData<[f64; N], &'a Point>> {
    let entries = points
        .iter()
        .map(|&p| GeomWithData::new(to_array::<N>(p), p))
        .collect::<Vec<_>>();
    RTree::bulk_load(entries)
}

impl<'a> RegionQuery<'a> for RTreeQueryEngine<'a> {
    fn init(&mut self, points: &'a HashSet<&'a Point>) {
        let Some(&first) = points.iter().next() else {
            self.inner = None;
            self.dim = 0;
            return;
        };

        let d = first.len();
        debug_assert!(points.iter().all(|&p| p.len() == d));

        self.inner = Some(match d {
            1 => RTreeAnyDim::D1(build_tree::<1>(points)),
            2 => RTreeAnyDim::D2(build_tree::<2>(points)),
            3 => RTreeAnyDim::D3(build_tree::<3>(points)),
            4 => RTreeAnyDim::D4(build_tree::<4>(points)),
            5 => RTreeAnyDim::D5(build_tree::<5>(points)),
            6 => RTreeAnyDim::D6(build_tree::<6>(points)),
            7 => RTreeAnyDim::D7(build_tree::<7>(points)),
            8 => RTreeAnyDim::D8(build_tree::<8>(points)),
            9 => RTreeAnyDim::D9(build_tree::<9>(points)),
            10 => RTreeAnyDim::D10(build_tree::<10>(points)),
            11 => RTreeAnyDim::D11(build_tree::<11>(points)),
            12 => RTreeAnyDim::D12(build_tree::<12>(points)),
            13 => RTreeAnyDim::D13(build_tree::<13>(points)),
            14 => RTreeAnyDim::D14(build_tree::<14>(points)),
            15 => RTreeAnyDim::D15(build_tree::<15>(points)),
            16 => RTreeAnyDim::D16(build_tree::<16>(points)),
            _ => panic!(
                "RTreeQueryEngine supports dimensions 1..=16; got {}. Consider using FakeQueryEngine or extend support.",
                d
            ),
        });
        self.dim = d;
    }

    fn run(&self, point: &'a Point, eps: f64) -> HashSet<&'a Point> {
        let Some(ref inner) = self.inner else {
            return HashSet::new();
        };

        assert_eq!(
            point.len(),
            self.dim,
            "query point dimension {} does not match tree dimension {}",
            point.len(),
            self.dim
        );

        let eps_sq = eps * eps;
        with_dim!(inner, |tree, N| {
            tree.locate_within_distance(to_array::<N>(point), eps_sq)
                .map(|it| it.data)
                .collect()
        })
    }

    fn k_dist(&self, point: &'a Point, k: usize) -> f64 {
        let Some(ref inner) = self.inner else {
            panic!("RTreeQueryEngine is not initialized");
        };
        assert!(k > 0, "k must be >= 1");

        with_dim!(inner, |tree, N| {
            let q = to_array::<N>(point);
            let mut seen = 0usize;
            for item in tree.nearest_neighbor_iter(&q) {
                let other = item.data;
                if std::ptr::eq(other, point) {
                    continue;
                }
                seen += 1;
                if seen == k {
                    return dist(point, other);
                }
            }
            panic!("k={} is out of range for dataset", k);
        })
    }
}
