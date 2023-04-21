use hydroflow::scheduled::graph::Hydroflow;

pub fn print_graph(flow: &Hydroflow) {//, graph: GraphType) {
    let serde_graph = flow
        .meta_graph()
        .expect("No graph found, maybe failed to parse.");
    // match graph {
    //     GraphType::Mermaid => {
    //         println!("{}", serde_graph.to_mermaid());
    //     }
    //     GraphType::Dot => {
    //         println!("{}", serde_graph.to_dot())
    //     }
    // }
    println!("{}", serde_graph.to_mermaid());
}
