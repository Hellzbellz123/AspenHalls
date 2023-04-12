use bevy::{prelude::Vec3, math::Vec3Swizzles};

use super::disjoint_set::DisjointSetUnion;

#[derive(Debug)]
pub struct Edge {
    source: Vec3,
    destination: Vec3,
    cost: i64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.destination == other.destination
            && self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Edge {
    fn new(source: Vec3, destination: Vec3, cost: i64) -> Self {
        Self {
            source,
            destination,
            cost,
        }
    }
}

pub fn kruskal(mut edges: Vec<Edge>, number_of_vertices: i64) -> (i64, Vec<Edge>) {
    let mut dsu = DisjointSetUnion::new(number_of_vertices as usize);

    edges.sort_unstable_by(|a, b| a.cost.cmp(&b.cost));
    let mut total_cost: i64 = 0;
    let mut final_edges: Vec<Edge> = Vec::new();
    let mut merge_count: i64 = 0;
    for edge in edges.iter() {
        if merge_count >= number_of_vertices - 1 {
            break;
        }

        let source: Vec3 = edge.source;
        let destination: Vec3 = edge.destination;
        if dsu.merge(source.yzxz() as usize, destination as usize) < usize::MAX {
            merge_count += 1;
            let cost: i64 = edge.cost;
            total_cost += cost;
            let final_edge: Edge = Edge::new(source, destination, cost);
            final_edges.push(final_edge);
        }
    }
    (total_cost, final_edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_vertices_eleven_edges() {
        let edges = vec![
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 1.0, y: 1.0, z: 0.0 }, 7),
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 5),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 2.0, y: 2.0, z: 0.0 }, 8),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 7),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 5),
            Edge::new(Vec3 { x: 3.0, y: 3.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 15),
            Edge::new(Vec3 { x: 3.0, y: 3.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 6),
            Edge::new(Vec3 { x: 4.0, y: 4.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 8),
            Edge::new(Vec3 { x: 4.0, y: 4.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 5.0, y: 5.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 11),
        ];

        let number_of_vertices: i64 = 7;

        let expected_total_cost = 39;
        let expected_used_edges = vec![
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 5),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 5),
            Edge::new(Vec3 { x: 3.0, y: 3.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 6),
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 1.0, y: 1.0, z: 0.0 }, 7),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 7),
            Edge::new(Vec3 { x: 4.0, y: 4.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 9),
        ];

        let (actual_total_cost, actual_final_edges) = kruskal(edges, number_of_vertices);

        assert_eq!(actual_total_cost, expected_total_cost);
        assert_eq!(actual_final_edges, expected_used_edges);
    }

    #[test]
    fn test_ten_vertices_twenty_edges() {
        let edges = vec![
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 1.0, y: 1.0, z: 0.0 }, 3),
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 6),
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 2.0, y: 2.0, z: 0.0 }, 2),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 4),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 4.0, y: 4.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 2),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 8),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 3.0, y: 3.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 4.0, y: 4.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 8),
            Edge::new(Vec3 { x: 4.0, y: 4.0, z: 0.0 }, Vec3 { x: 9.0, y: 9.0, z: 0.0 }, 18),
            Edge::new(Vec3 { x: 5.0, y: 5.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 7),
            Edge::new(Vec3 { x: 5.0, y: 5.0, z: 0.0 }, Vec3 { x: 8.0, y: 8.0, z: 0.0 }, 9),
            Edge::new(Vec3 { x: 5.0, y: 5.0, z: 0.0 }, Vec3 { x: 9.0, y: 9.0, z: 0.0 }, 10),
            Edge::new(Vec3 { x: 6.0, y: 6.0, z: 0.0 }, Vec3 { x: 7.0, y: 7.0, z: 0.0 }, 4),
            Edge::new(Vec3 { x: 6.0, y: 6.0, z: 0.0 }, Vec3 { x: 8.0, y: 8.0, z: 0.0 }, 5),
            Edge::new(Vec3 { x: 7.0, y: 7.0, z: 0.0 }, Vec3 { x: 8.0, y: 8.0, z: 0.0 }, 1),
            Edge::new(Vec3 { x: 7.0, y: 7.0, z: 0.0 }, Vec3 { x: 9.0, y: 9.0, z: 0.0 }, 4),
            Edge::new(Vec3 { x: 8.0, y: 8.0, z: 0.0 }, Vec3 { x: 9.0, y: 9.0, z: 0.0 }, 3),
        ];

        let number_of_vertices: i64 = 10;

        let expected_total_cost = 38;
        let expected_used_edges = vec![
            Edge::new(Vec3 { x: 7.0, y: 7.0, z: 0.0 }, Vec3 { x: 8.0, y: 8.0, z: 0.0 }, 1),
            Edge::new(Vec3 { x: 1.0, y: 1.0, z: 0.0 }, Vec3 { x: 2.0, y: 2.0, z: 0.0 }, 2),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 3.0, y: 3.0, z: 0.0 }, 2),
            Edge::new(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3 { x: 1.0, y: 1.0, z: 0.0 }, 3),
            Edge::new(Vec3 { x: 8.0, y: 8.0, z: 0.0 }, Vec3 { x: 9.0, y: 9.0, z: 0.0 }, 3),
            Edge::new(Vec3 { x: 6.0, y: 6.0, z: 0.0 }, Vec3 { x: 7.0, y: 7.0, z: 0.0 }, 4),
            Edge::new(Vec3 { x: 5.0, y: 5.0, z: 0.0 }, Vec3 { x: 6.0, y: 6.0, z: 0.0 }, 7),
            Edge::new(Vec3 { x: 2.0, y: 2.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 8),
            Edge::new(Vec3 { x: 4.0, y: 4.0, z: 0.0 }, Vec3 { x: 5.0, y: 5.0, z: 0.0 }, 8),
        ];

        let (actual_total_cost, actual_final_edges) = kruskal(edges, number_of_vertices);

        assert_eq!(actual_total_cost, expected_total_cost);
        assert_eq!(actual_final_edges, expected_used_edges);
    }
}