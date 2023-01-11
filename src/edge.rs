use std::fmt::Display;

use uuid::Uuid;

use crate::{
    graph::Graph,
    traversal::Traversal,
    vertex::{Vertex, VertexList},
};

#[derive(Clone, Debug)]
pub struct EdgeList<'a>(pub Vec<Edge<'a>>);

impl EdgeList<'_> {
    pub fn from(&mut self, vertex_list: Traversal<VertexList>) {
        todo!()
    }

    pub fn to(&mut self, vertex_list: Traversal<VertexList>) {
        todo!()
    }

    pub fn drop(&mut self, graph: &mut Graph) {
        todo!()
    }
}

// impl std::ops::Deref for EdgeList {
//     type Target<'a> = Vec<Edge<'a>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl std::ops::DerefMut for EdgeList {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl Display for EdgeList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, edge| {
            result.and_then(|_| writeln!(f, "{}", edge))
        })
    }
}

#[derive(Clone, Debug)]
pub struct Edge<'a> {
    pub id: Uuid,
    pub from: Option<&'a Vertex<'a>>,
    pub to: Option<&'a Vertex<'a>>,
    pub name: String,
}

impl Edge<'_> {
    pub fn new(name: &str) -> Edge {
        Edge {
            id: Uuid::new_v4(),
            from: None,
            to: None,
            name: String::from(name),
        }
    }
}

impl Display for Edge<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        if let Some(from) = &self.from {
            str.push_str(from.to_string().as_str())
        }
        str.push_str(format!(" -> {} -> ", self.name).as_str());

        if let Some(to) = &self.to {
            str.push_str(to.to_string().as_str());
        }
        write!(f, "{}", str)
    }
}
