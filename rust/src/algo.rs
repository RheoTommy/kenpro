use crate::types::{Class, Point};
use std::collections::{HashMap, HashSet};

pub trait RegionQuery<'a> {
    fn init(&mut self, points: &'a HashSet<&'a Point>);
    fn run(&self, point: &'a Point, eps: f64) -> HashSet<&'a Point>;
    fn k_dist(&self, point: &'a Point, k: usize) -> f64;
}

pub struct Algo<'a, T: RegionQuery<'a>> {
    region_query: &'a mut T,
    points: &'a HashSet<&'a Point>,
    eps: f64,
    min_pts: usize,
}

impl<'a, T: RegionQuery<'a>> Algo<'a, T> {
    pub fn new(
        region_query: &'a mut T,
        points: &'a HashSet<&'a Point>,
        eps: f64,
        min_pts: usize,
    ) -> Self {
        region_query.init(&points);

        Self {
            region_query,
            points,
            eps,
            min_pts,
        }
    }

    pub fn dbscan(&self) -> HashMap<&'a Point, Class> {
        let mut classes = self
            .points
            .iter()
            .map(|&p| (p, Class::Unclassified))
            .collect::<HashMap<_, _>>();

        let mut cluster_id = 0;

        for &p in self.points.iter() {
            match classes[p] {
                Class::Classified(_) | Class::Noise => continue,
                Class::Unclassified => {
                    if self.expand_cluster(p, cluster_id, &mut classes) {
                        cluster_id += 1;
                    }
                }
            }
        }

        classes
    }

    // Main DFS entrypoint.
    fn expand_cluster(
        &self,
        point: &'a Point,
        cluster_id: usize,
        classes: &mut HashMap<&'a Point, Class>,
    ) -> bool {
        let neighbors = self.region_query.run(point, self.eps);

        // This point can't be a core point.
        if neighbors.len() < self.min_pts {
            // It is marked as Noise for now, but it can be a border point later.
            if let Some(old) = classes.insert(point, Class::Noise) {
                assert_eq!(
                    old,
                    Class::Unclassified,
                    "The entry should be unclassified here."
                );
            }
            return false;
        }

        // This point is a core point of a cluster {cluster_id}.

        // Mark neighbors that are currently unassigned/noise as classified.
        for &p in neighbors.iter() {
            match classes[p] {
                Class::Unclassified | Class::Noise => {
                    classes.insert(p, Class::Classified(cluster_id));
                }
                Class::Classified(_) => {
                    // Already assigned: leave as-is.
                }
            }
        }

        let mut set = neighbors;
        set.remove(point);
        // Sub loop to expand the cluster.
        while !set.is_empty() {
            let current_point = *set.iter().next().unwrap();
            let neighbors = self.region_query.run(current_point, self.eps);

            // If current_point is a core point.
            if neighbors.len() >= self.min_pts {
                for &p in neighbors.iter() {
                    match classes[p] {
                        Class::Classified(_cid) => {
                            // Already assigned. If it belongs to a different cluster,
                            // leave it unchanged.
                        }
                        Class::Unclassified => {
                            // Check neighbors of this point recursively.
                            set.insert(p);
                            classes.insert(p, Class::Classified(cluster_id));
                        }
                        Class::Noise => {
                            // Include as border point.
                            classes.insert(p, Class::Classified(cluster_id));
                        }
                    }
                }
            }
            set.remove(current_point);
        }

        true
    }
}
