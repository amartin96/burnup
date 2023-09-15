use graphviz_rust::{
    dot_generator::{graph, id, node},
    dot_structures::{Graph, Id, Node, NodeId, Stmt},
};
use ::Nuclide::{Atom, decay::DecayMode, Nuclide};

fn graph_decay_series_inner(graph: &mut Graph, nuclide: &Nuclide) {
    // Add node for each daughter nuclide
    nuclide.decay_mode();

    // Recurse for all daughter nuclides
}

fn graph_decay_series(nuclide: &Nuclide) -> Graph {
    let mut graph = graph!(strict di id!(format!("{} Decay Series", nuclide.to_string())));

    // Add root node
    graph.add_stmt(Stmt::Node(node!(nuclide.to_string())));
    graph_decay_series_inner(&mut graph, nuclide);

    graph
}

fn main() {
    println!("Hello, world!");
}
