use itertools::Itertools;

use crate::{
    fastset::{DenseFastSet, FastSet},
    graph::{Graph, Vertex},
};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub enum OperationKind {
    Select,
    Exclude,
    Ignore,
}

#[derive(Debug)]
pub struct Operation {
    v: Vertex,
    color: Color,
    kind: OperationKind,
}

#[derive(Debug)]
pub struct SolverContext {
    pub graph: Graph,
    pub history: Vec<Operation>,
    solution: DenseFastSet<Vertex>,
    /// Vertices that are already dominated
    white: DenseFastSet<Vertex>,
    /// Vertices that can't belong to solution, but need to be dominated
    excluded: FastSet<Vertex>,
    /// How many vertices in neighborhood that need to be dominated
    black_neighbors: Vec<u32>,
    /// How many vertices in neighborhood that are dominated
    white_neighbors: Vec<u32>,
    /// How many vertices in neighborhood that are in solution
    dom_amount: Vec<u32>,
}

impl SolverContext {
    pub fn new(graph: Graph) -> Self {
        let size = graph.size();
        let black_neighbors = graph.neighbors.iter().map(|n| n.len() as u32).collect_vec();
        Self {
            graph,
            history: Vec::default(),
            solution: DenseFastSet::new(size),
            white: DenseFastSet::new(size),
            excluded: FastSet::new(size),
            // TODO intialize these properly
            black_neighbors,
            white_neighbors: vec![0; size],
            dom_amount: vec![0; size],
        }
    }

    /// Remove vertex from its color set and return color it belonged to
    pub fn add_color(&mut self, v: Vertex, color: Color) {
        assert!(!self.white.contains(v));
        if let Color::White = color {
            self.white.insert_unchecked(v)
        }
    }

    /// Remove vertex from its color set and return color it belonged to
    pub fn remove_color(&mut self, v: Vertex) -> Color {
        if self.white.remove(v) {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Return color of vertex
    pub fn get_color(&mut self, v: Vertex) -> Color {
        if self.white.contains(v) {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Take vertex `v` into the solution
    pub fn select(&mut self, v: Vertex) {
        assert!(!self.excluded.contains(v));
        assert!(!self.solution.contains(v));
        self.solution.insert_unchecked(v);
        self.graph.invalidate(v);
        // Remove vertex from color set and get its color
        let color = self.remove_color(v);
        for n in self.graph.neighbors(v) {
            match color {
                Color::White => self.white_neighbors[n] -= 1,
                Color::Black => self.black_neighbors[n] -= 1,
            }
            self.dom_amount[n] += 1;
            assert!(self.dom_amount[n] > 0);
            // If dominated for the first time change vertex color from black to white
            if self.dom_amount[n] == 1 {
                assert!(!self.white.contains(n));
                self.white.insert_unchecked(n);
                for n2 in self.graph.neighbors(n) {
                    self.white_neighbors[n2] += 1;
                }
            }
        }
        self.history.push(Operation {
            v,
            color,
            kind: OperationKind::Select,
        });
    }

    pub fn undo_select(&mut self, v: Vertex, color: Color) {
        self.solution.remove(v);
        self.graph.revalidate(v);
        self.add_color(v, color);
        for n in self.graph.neighbors(v) {
            match color {
                Color::White => self.white_neighbors[n] += 1,
                Color::Black => self.black_neighbors[n] += 1,
            }
            assert!(self.dom_amount[n] > 0);
            self.dom_amount[n] -= 1;
            if self.dom_amount[n] == 0 {
                assert!(self.white.contains(n));
                self.white.remove(n);
                for n2 in self.graph.neighbors(n) {
                    self.white_neighbors[n2] -= 1;
                }
            }
        }
    }

    pub fn exclude(&mut self, v: Vertex) {
        let color = self.get_color(v);
        // If vertex to be included is white remove it.
        // If it is black keep it, because it still needs to be dominated
        // and add to excluded set
        match color {
            Color::White => {
                self.white.remove(v);
                self.graph.invalidate(v);
                for n in self.graph.neighbors(v) {
                    self.white_neighbors[n] -= 1;
                }
            }
            Color::Black => self.excluded.insert(v),
        };
        self.history.push(Operation {
            v,
            color,
            kind: OperationKind::Exclude,
        });
    }

    pub fn undo_exclude(&mut self, v: Vertex, color: Color) {
        self.graph.revalidate(v);
        match color {
            Color::White => {
                self.white.insert_unchecked(v);
                self.graph.revalidate(v);
                for n in self.graph.neighbors(v) {
                    self.white_neighbors[n] += 1;
                }
            }
            Color::Black => {
                self.excluded.remove(v);
            }
        }
    }

    pub fn rollback(&mut self, time: usize) {
        while let Some(op) = self.history.pop() {
            match op.kind {
                OperationKind::Select => self.undo_select(op.v, op.color),
                OperationKind::Exclude => self.undo_exclude(op.v, op.color),
                OperationKind::Ignore => todo!(),
            }

            if self.history.len() == time {
                break;
            }
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn select() {
        let edges = "4\n0 1\n1 2\n2 3\n3 0\n";
        let graph = Graph::new_from_edges(edges.to_string());
        let mut context = SolverContext::new(graph);
        [0, 2].iter().for_each(|&v| {
            context.select(v);
            assert!(context.solution.contains(v));
            assert!(!context.graph.is_valid(v));
            for n in &context.graph.neighbors[v] {
                assert!(context.white.contains(*n));
            }
        });

        [1, 3].iter().for_each(|&v| {
            assert!(context.dom_amount[v] == 2);
        });
    }

    #[test]
    fn undo_select() {
        let edges = "4\n0 1\n1 2\n2 3\n3 0\n";
        let graph = Graph::new_from_edges(edges.to_string());
        let mut context = SolverContext::new(graph);
        let v = 0;
        context.select(v);
        context.undo_select(v, Color::Black);
        assert!(context.solution.is_empty());
        assert!(context.white.is_empty());
        assert!(context.white_neighbors.iter().all(|&wn| wn == 0));
    }
}
