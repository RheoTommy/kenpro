use crate::algo::RegionQuery;
use crate::types::{dist, Point};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct FakeQueryEngine<'a> {
    sorted_by_distance: HashMap<&'a Point, Vec<&'a Point>>,
}

impl<'a> FakeQueryEngine<'a> {
    pub fn new() -> Self {
        Self {
            sorted_by_distance: HashMap::new(),
        }
    }
}

impl<'a> RegionQuery<'a> for FakeQueryEngine<'a> {
    // This will take O(N^2 logN) for initialization.
    fn init(&mut self, points: &'a HashSet<&'a Point>) {
        let mut sorted_by_distance = HashMap::new();

        for &point in points.iter() {
            let sorted = points
                .iter()
                .sorted_by(|&&a, &&b| {
                    let a_dist = dist(a, point);
                    let b_dist = dist(b, point);
                    a_dist.partial_cmp(&b_dist).unwrap()
                })
                .map(|&p| p)
                .collect_vec();

            sorted_by_distance.insert(point, sorted);
        }

        self.sorted_by_distance = sorted_by_distance;
    }

    fn run(&self, point: &'a Point, eps: f64) -> HashSet<&'a Point> {
        assert_ne!(
            self.sorted_by_distance.get(point),
            None,
            "The query engine is not initialized for this point."
        );

        let sorted = self.sorted_by_distance.get(point).unwrap();

        let mut lt = 0;
        let mut ge = sorted.len();
        while ge - lt > 1 {
            let mid = (lt + ge) / 2;
            if dist(&sorted[mid], &point) <= eps {
                lt = mid;
            } else {
                ge = mid;
            }
        }

        sorted.iter().take(ge).cloned().collect()
    }
}
