use crate::context::SolverContext;

pub struct Solver {
    context: SolverContext,
    num_nodes: u64,
}

impl Solver {
    pub fn branch(&mut self) {
        self.num_nodes += 1;
        let Some(candidate) = self
            .context
            .graph
            .valid
            .iter()
            .min_by_key(|v| self.context.graph.neighbors(*v).count())
        else {
            return;
        };
        let candidates = self.context.graph.neighbors[candidate].clone();
        for n in candidates {
            let time = self.context.history.len();
            self.context.select(n);
            self.branch();
            self.context.rollback(time);
            self.context.exclude(n);
        }
    }
}
