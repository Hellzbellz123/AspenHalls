use bevy::math::Vec2;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
struct Vertex {
    position: Vec2,
}

impl Vertex {
    fn new(position: Vec2) -> Self {
        Vertex { position }
    }
}

impl Eq for Vertex {}

// impl PartialEq for Vertex {
//     fn eq(&self, other: &Self) -> bool {
//         self.position == other.position
//     }
// }

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self;
        let b = other;

        let x = a.position.x.partial_cmp(&b.position.x);
        let y = a.position.y.partial_cmp(&b.position.y);

        Some(self.position.cmp(&other.position))
    }
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.x.to_bits().hash(state);
        self.position.y.to_bits().hash(state);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
    is_bad: bool,
}

impl Triangle {
    fn new(a: Vertex, b: Vertex, c: Vertex) -> Self {
        Triangle {
            a,
            b,
            c,
            is_bad: false,
        }
    }

    fn contains_vertex(&self, v: Vec2) -> bool {
        v.distance(self.a.position) < 0.01
            || v.distance(self.b.position) < 0.01
            || v.distance(self.c.position) < 0.01
    }

    fn circum_circle_contains(&self, v: Vec2) -> bool {
        let a = self.a.position;
        let b = self.b.position;
        let c = self.c.position;

        let ab = a.length_squared();
        let cd = b.length_squared();
        let ef = c.length_squared();

        let circum_x = (ab * (c.y - b.y) + cd * (a.y - c.y) + ef * (b.y - a.y))
            / (a.x * (c.y - b.y) + b.x * (a.y - c.y) + c.x * (b.y - a.y));
        let circum_y = (ab * (c.x - b.x) + cd * (a.x - c.x) + ef * (b.x - a.x))
            / (a.y * (c.x - b.x) + b.y * (a.x - c.x) + c.y * (b.x - a.x));

        let circum = Vec2::new(circum_x / 2.0, circum_y / 2.0);
        let circum_radius = (a - circum).length_squared();
        let dist = (v - circum).length_squared();

        dist <= circum_radius
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Edge {
    u: Vertex,
    v: Vertex,
    is_bad: bool,
}
use std::hash::Hash;
use std::hash::Hasher;

struct Delaunay2D {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    triangles: Vec<Triangle>,
}

impl Delaunay2D {
    fn new(vertices: Vec<Vertex>) -> Self {
        Delaunay2D {
            vertices,
            edges: Vec::new(),
            triangles: Vec::new(),
        }
    }

    fn triangulate_from_vertex(vertices: Vec<Vertex>) -> Self {
        let mut delaunay = Delaunay2D::new(vertices.clone());
        delaunay.triangulate_self();
        delaunay
    }

    fn triangulate_self(&mut self) {
        let mut min_x = self.vertices[0].position.x;
        let mut min_y = self.vertices[0].position.y;
        let mut max_x = min_x;
        let mut max_y = min_y;

        for vertex in &self.vertices {
            let position = vertex.position;
            if position.x < min_x {
                min_x = position.x;
            }
            if position.x > max_x {
                max_x = position.x;
            }
            if position.y < min_y {
                min_y = position.y;
            }
            if position.y > max_y {
                max_y = position.y;
            }
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let delta_max = dx.max(dy) * 2.0;

        let p1 = Vertex::new(Vec2::new(min_x - 1.0, min_y - 1.0));
        let p2 = Vertex::new(Vec2::new(min_x - 1.0, max_y + delta_max));
        let p3 = Vertex::new(Vec2::new(max_x + delta_max, min_y - 1.0));

        self.triangles
            .push(Triangle::new(p1.clone(), p2.clone(), p3.clone()));

        for vertex in &self.vertices {
            let mut polygon = Vec::new();

            for triangle in &self.triangles {
                if triangle.circum_circle_contains(vertex.position) {
                    triangle.is_bad = true;
                    polygon.push(Edge::new(triangle.a.clone(), triangle.b.clone()));
                    polygon.push(Edge::new(triangle.b.clone(), triangle.c.clone()));
                    polygon.push(Edge::new(triangle.c.clone(), triangle.a.clone()));
                }
            }

            self.triangles.retain(|t| !t.is_bad);

            for i in 0..polygon.len() {
                for j in i + 1..polygon.len() {
                    if polygon[i].almost_equal(&polygon[j]) {
                        polygon[i].is_bad = true;
                        polygon[j].is_bad = true;
                    }
                }
            }

            polygon.retain(|e| !e.is_bad);

            for edge in polygon {
                self.triangles.push(Triangle::new(
                    edge.u.clone(),
                    edge.v.clone(),
                    vertex.clone(),
                ));
            }
        }

        self.triangles.retain(|t| {
            !t.contains_vertex(p1.position)
                && !t.contains_vertex(p2.position)
                && !t.contains_vertex(p3.position)
        });

        let mut edge_set = HashSet::new();

        for triangle in &self.triangles {
            let ab = Edge::new(triangle.a.clone(), triangle.b.clone());
            let bc = Edge::new(triangle.b.clone(), triangle.c.clone());
            let ca = Edge::new(triangle.c.clone(), triangle.a.clone());

            if edge_set.insert(ab.clone()) {
                self.edges.push(ab);
            }

            if edge_set.insert(bc.clone()) {
                self.edges.push(bc);
            }

            if edge_set.insert(ca.clone()) {
                self.edges.push(ca);
            }
        }
    }
}

fn main() {
    let vertices = vec![
        Vertex::new(Vec2::new(0.0, 0.0)),
        Vertex::new(Vec2::new(1.0, 0.0)),
        Vertex::new(Vec2::new(0.5, 0.5)),
    ];

    let delaunay = Delaunay2D::triangulate_from_vertex(vertices);

    println!("{:?}", delaunay.triangles);
    println!("{:?}", delaunay.edges);
}
