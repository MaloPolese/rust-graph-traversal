use traversal::Traversal;
use vertex::VertexList;

mod edge;
mod graph;
mod traversal;
mod vertex;

fn main() {
    let mut graph = graph::Graph::new();
    let t: Traversal<VertexList> = Traversal::new(&graph);

    let list = VertexList(Vec::new());

    let res = t.add_e().add_e().add_e().next(list);
}
