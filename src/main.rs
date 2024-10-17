use context::SolverContext;
use graph::Graph;
use solver::Solver;

pub mod context;
pub mod fastset;
pub mod graph;
pub mod solver;

fn main() {
    let edges = "4\n0 1\n1 2\n2 3\n3 0\n";
    let graph = Graph::new_from_edges(edges.to_string());
    let context = SolverContext::new(graph);
    let mut solver = Solver::new(context);
    solver.branch();
    println!("SOLUTION");
    solver.print_best();
}
