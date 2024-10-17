use itertools::Itertools;

use crate::{
    context::SolverContext,
    graph::{Graph, Vertex},
};

pub struct Solver {
    context: SolverContext,
    best_solution: Vec<Vertex>,
    num_nodes: u64,
}

impl Solver {
    pub fn new(context: SolverContext) -> Self {
        let size = context.graph.size();
        Self {
            context,
            best_solution: (0..size).collect_vec(),
            num_nodes: 0,
        }
    }
    pub fn branch(&mut self) {
        self.num_nodes += 1;
        if self.context.is_dominated() {
            if self.context.solution.size() < self.best_solution.len() {
                self.best_solution = self.context.solution.iter().collect_vec();
            }
        }
        let Some(candidate) = self
            .graph()
            .valid
            .iter()
            .filter(|v| !self.context.white.contains(*v))
            .min_by_key(|v| self.context.graph.valid_neighbors(*v).count())
        else {
            return;
        };
        let candidates = self.context.graph.valid_neighbors(candidate).collect_vec();
        for n in candidates {
            let time = self.context.history.len();
            self.context.select(n);
            self.branch();
            self.context.rollback(time);
            self.context.exclude(n);
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.context.graph
    }

    pub fn print_best(&self) {
        for v in &self.best_solution {
            println!("{v}");
        }
    }
}
