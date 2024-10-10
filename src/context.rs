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
    Relax,
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
    white: DenseFastSet<Vertex>,
    black: DenseFastSet<Vertex>,
    black_neighbors: Vec<u32>,
    white_neighbors: Vec<u32>,
}

impl SolverContext {
    pub fn remove(&mut self, v: Vertex) -> Color {
        let black = self.black.remove(v);
        let white = self.white.remove(v);
        assert!(black != white);
        if black {
            Color::Black
        } else {
            Color::White
        }
    }

    /// Take vertex `v` into the solution
    pub fn select(&mut self, v: Vertex) {
        self.solution.insert(v);
        self.graph.invalidate(v);
        // Remove vertex from color set and get its color
        let color = self.remove(v);
        for n in self.graph.neighbors[v.0].0.iter() {
            match color {
                Color::White => self.white_neighbors[n.0] -= 1,
                Color::Black => self.black_neighbors[n.0] -= 1,
            }
            self.black.remove(*n);
            let was_black = self.white.insert(*n);
            if was_black {
                for n2 in self.graph.neighbors[n.0].0.iter() {
                    self.white_neighbors[n2.0] += 1;
                }
            }
        }
    }

    pub fn undo_select(&mut self, v: Vertex, color: Color) {
        self.solution.remove(v);
        self.black.insert(v);
        self.graph.revalidate(v);
        for n in self.graph.neighbors[v.0].0.iter() {
            self.black.insert(*n);
            self.white.remove(*n);
            for n2 in self.graph.neighbors[n.0].0.iter() {
                self.white_neighbors[n2.0] -= 1;
            }
        }
    }

    pub fn exclude(&mut self, v: Vertex) {
        self.graph.invalidate(v);
        let black = self.black.remove(v);
        let white = self.white.remove(v);
        assert!(black != white);
        let color = if black { Color::Black } else { Color::White };
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
        }
    }
}
