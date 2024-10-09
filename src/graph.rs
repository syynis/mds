use std::collections::HashMap;

use crate::fastset::DenseFastSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vertex(usize);
impl From<Vertex> for usize {
    fn from(value: Vertex) -> Self {
        value.0
    }
}

pub struct Adjacency(Vec<Vertex>);

pub enum VertexKind {
    Black,
    White,
}

pub struct Graph {
    valid: DenseFastSet<Vertex>,
    neighbors: Vec<Adjacency>,
    kinds: Vec<VertexKind>,
    names: Vec<String>,
    id_name_map: HashMap<String, usize>,
    num_vertices: u32,
}

impl Graph {
    pub fn invalidate_vertex(&mut self, v: Vertex) {
        self.valid.remove(v);
        self.num_vertices -= 1;
    }

    pub fn revalidate_vertex(&mut self, v: Vertex) {
        self.valid.insert(v);
        self.num_vertices += 1;
    }

    pub fn add_edge(&mut self, v: Vertex, u: Vertex) {
        debug_assert!(self.is_valid(v));
        debug_assert!(self.is_valid(u));
        self.neighbors[v.0].0.push(u);
        self.neighbors[u.0].0.push(v);
    }

    pub fn has_edge(&self, v: Vertex, u: Vertex) -> bool {
        debug_assert!(self.is_valid(v));
        debug_assert!(self.is_valid(u));
        if self.neighbors[v.0].0.len() < self.neighbors[u.0].0.len() {
            self.neighbors[v.0].0.iter().any(|&n| n == u)
        } else {
            self.neighbors[u.0].0.iter().any(|&n| n == v)
        }
    }

    pub fn is_valid(&self, v: Vertex) -> bool {
        self.valid.contains(v)
    }
}
