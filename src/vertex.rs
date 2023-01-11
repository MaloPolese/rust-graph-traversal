use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

use crate::{edge::Edge, graph::Graph, traversal::StepFn};

pub trait IVertex {
    fn out_e(&self);

    fn in_e(&self);

    fn out_v(&self);

    fn in_v(&self);

    fn add_edge<'a, T>(name: &str) -> StepFn<'a, T>
    where
        T: IVertex + 'a;

    fn add_vertex(&mut self, name: &str, graph: &mut Graph);

    fn drop(&mut self, graph: &mut Graph);
}

#[derive(Clone, Debug)]
pub struct VertexList<'a>(pub Vec<Vertex<'a>>);
impl IVertex for VertexList<'_> {
    fn out_e(&self) {
        todo!()
    }

    fn in_e(&self) {
        todo!()
    }

    fn out_v(&self) {
        todo!()
    }

    fn in_v(&self) {
        todo!()
    }

    // fn expand<'a>() -> StepFn<'a> {
    //     Box::new(move |traversal, g| {
    //         let mut result = Vec::new();
    //         for vertex in traversal {
    //             result.extend(
    //                 g.edges.
    //                     iter()
    //                     .filter(|e| e.source == vertex)
    //                     .map(|e| e.destination.clone())
    //             )
    //         }
    //         result
    //     })
    // }

    fn add_edge<'a, T>(name: &str) -> StepFn<'a, T>
    where
        T: IVertex + 'a,
    {
        Box::new(move |traversal, graph| {
            graph.vertices.0.push(Vertex::new("name"));
            println!("------------- Add edge -------------");
            traversal
        })
    }

    fn add_vertex(&mut self, name: &str, graph: &mut Graph) {
        todo!()
    }

    fn drop(&mut self, graph: &mut Graph) {
        todo!()
    }
}

// impl std::ops::Deref for VertexList {
//     type Target = Vec<Vertex>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl std::ops::DerefMut for VertexList {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl Display for VertexList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, vertex| {
            result.and_then(|_| writeln!(f, "{}", vertex))
        })
    }
}

#[derive(Clone, Debug)]
pub struct Vertex<'a> {
    pub id: Uuid,
    pub name: String,
    out_e: Vec<&'a Edge<'a>>,
    in_e: Vec<&'a Edge<'a>>,
    pub properties: HashMap<String, String>,
}

impl Vertex<'_> {
    pub fn new(name: &str) -> Vertex {
        Vertex {
            id: Uuid::new_v4(),
            name: String::from(name),
            out_e: Vec::new(),
            in_e: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

impl Display for Vertex<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} | {}]", self.id, self.name)
    }
}
