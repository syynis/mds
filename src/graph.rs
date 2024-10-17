use std::{collections::HashMap, fs::read_to_string, path::Path};

use itertools::Itertools;

use crate::fastset::DenseFastSet;

pub type Vertex = usize;
pub type Adjacency = Vec<Vertex>;

#[derive(Debug)]
pub struct Graph {
    pub valid: DenseFastSet<Vertex>,
    pub neighbors: Vec<Adjacency>,
    names: Vec<String>,
    id_name_map: HashMap<String, usize>,
    num_vertices: usize,
}

impl Graph {
    pub fn new_from_edges(content: String) -> Self {
        let lines = content.lines().collect_vec();
        let num_vertices = lines[0].trim().parse::<usize>().unwrap();
        let mut current_vertex = 0;
        let mut valid = DenseFastSet::new(num_vertices);
        let mut names = Vec::with_capacity(num_vertices);
        let mut id_name_map: HashMap<String, usize> = HashMap::with_capacity(num_vertices);
        let mut neighbors = vec![Vec::new(); num_vertices];
        for line in lines.iter().skip(1) {
            let edge = line.trim().split(' ').collect_vec();
            assert!(edge.len() == 2);
            let v = *id_name_map.entry(edge[0].to_owned()).or_insert_with(|| {
                names.push(edge[0].to_owned());
                valid.insert_unchecked(current_vertex);
                current_vertex += 1;
                current_vertex - 1
            });
            let u = *id_name_map.entry(edge[1].to_owned()).or_insert_with(|| {
                names.push(edge[1].to_owned());
                valid.insert_unchecked(current_vertex);
                current_vertex += 1;
                current_vertex - 1
            });
            neighbors[v].push(u);
            neighbors[u].push(v);
        }

        Self {
            valid,
            neighbors,
            names,
            id_name_map,
            num_vertices,
        }
    }

    pub fn new_from_file(file: &Path) -> Self {
        let content = read_to_string(file).unwrap();
        Self::new_from_edges(content)
    }

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

    pub fn size(&self) -> usize {
        self.num_vertices
    }

    pub fn neighbors(&self, v: Vertex) -> impl Iterator<Item = Vertex> + '_ {
        self.neighbors[v].iter().copied()
    }

    pub fn valid_neighbors(&self, v: Vertex) -> impl Iterator<Item = Vertex> + '_ {
        self.neighbors[v]
            .iter()
            .copied()
            .filter(|n| self.is_valid(*n))
    }
}
