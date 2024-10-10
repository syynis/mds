use std::collections::HashMap;

use crate::fastset::DenseFastSet;

pub type Vertex = usize;
pub type Adjacency = Vec<Vertex>;

#[derive(Debug)]
pub struct Graph {
    valid: DenseFastSet<Vertex>,
    pub neighbors: Vec<Adjacency>,
    names: Vec<String>,
    id_name_map: HashMap<String, usize>,
    num_vertices: u32,
}

impl Graph {
    pub fn invalidate(&mut self, v: Vertex) {
        self.valid.remove(v);
        self.num_vertices -= 1;
    }

    pub fn revalidate(&mut self, v: Vertex) {
        self.valid.insert(v);
        self.num_vertices += 1;
    }

    pub fn add_edge(&mut self, v: Vertex, u: Vertex) {
        debug_assert!(self.is_valid(v));
        debug_assert!(self.is_valid(u));
        self.neighbors[v].push(u);
        self.neighbors[u].push(v);
    }

    pub fn has_edge(&self, v: Vertex, u: Vertex) -> bool {
        debug_assert!(self.is_valid(v));
        debug_assert!(self.is_valid(u));
        if self.neighbors[v].len() < self.neighbors[u].len() {
            self.neighbors[v].iter().any(|&n| n == u)
        } else {
            self.neighbors[u].iter().any(|&n| n == v)
        }
    }

    pub fn is_valid(&self, v: Vertex) -> bool {
        self.valid.contains(v)
    }
}
