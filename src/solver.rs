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

        for n in self.context.graph.neighbors(candidate) {}
    }
}
