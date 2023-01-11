use crate::{edge::EdgeList, vertex::VertexList};

pub struct Graph<'a> {
    pub vertices: VertexList<'a>,
    pub edges: EdgeList<'a>,
}

impl Graph<'_> {
    pub fn new() -> Graph<'static> {
        Graph {
            vertices: VertexList(Vec::new()),
            edges: EdgeList(Vec::new()),
        }
    }
}
