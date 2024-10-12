use crate::{
    fastset::{DenseFastSet, FastSet},
    graph::{Graph, Vertex},
};

#[derive(Debug)]
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
    graph: Graph,
    operations: Vec<Operation>,
    solution: DenseFastSet<Vertex>,
    /// Vertices that are already dominated
    white: DenseFastSet<Vertex>,
    /// Vertices that are not dominated
    black: DenseFastSet<Vertex>,
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
        Self {
            graph,
            operations: Vec::default(),
            solution: DenseFastSet::new(size),
            white: DenseFastSet::new(size),
            black: DenseFastSet::new(size),
            excluded: FastSet::new(size),
            // TODO intialize these properly
            black_neighbors: vec![0; size],
            white_neighbors: vec![0; size],
            dom_amount: vec![0; size],
        }
    }

    /// Remove vertex from its color set and return color it belonged to
    pub fn remove_color(&mut self, v: Vertex) -> Color {
        assert!(self.black.contains(v) != self.white.contains(v));
        if self.black.remove(v) {
            Color::Black
        } else {
            self.white.remove(v);
            Color::White
        }
    }
    /// Return color of vertex
    pub fn get_color(&mut self, v: Vertex) -> Color {
        assert!(self.black.contains(v) != self.white.contains(v));
        if self.black.contains(v) {
            Color::Black
        } else {
            Color::White
        }
    }

    /// Take vertex `v` into the solution
    pub fn select(&mut self, v: Vertex) {
        assert!(!self.excluded.contains(v));
        assert!(!self.solution.contains(v));
        self.solution.insert(v);
        self.graph.invalidate(v);
        // Remove vertex from color set and get its color
        let color = self.remove_color(v);
        for n in self.graph.neighbors(v) {
            match color {
                Color::White => self.white_neighbors[n] -= 1,
                Color::Black => self.black_neighbors[n] -= 1,
            }
            self.dom_amount[n] += 1;
            self.black.remove(n);
            let was_black = self.white.insert(n);
            if was_black {
                for n2 in self.graph.neighbors(n) {
                    self.white_neighbors[n2] += 1;
                }
            }
        }
        self.operations.push(Operation {
            v,
            color,
            kind: OperationKind::Select,
        });
    }

    pub fn undo_select(&mut self, v: Vertex, color: Color) {
        self.solution.remove(v);
        self.black.insert(v);
        self.graph.revalidate(v);
        for n in self.graph.neighbors(v) {
            self.dom_amount[n] -= 1;
            self.black.insert(n);
            self.white.remove(n);
            for n2 in self.graph.neighbors(n) {
                self.white_neighbors[n2] -= 1;
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
        self.operations.push(Operation {
            v,
            color,
            kind: OperationKind::Exclude,
        });
    }

    pub fn undo_exclude(&mut self, v: Vertex, color: Color) {
        self.graph.revalidate(v);
        match color {
            Color::White => self.white.insert(v),
            Color::Black => self.black.insert(v),
        };
    }
}
