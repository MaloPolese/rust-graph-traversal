use std::{fmt::Display, rc::Rc};

use crate::{
    graph::Graph,
    vertex::{IVertex, Vertex, VertexList},
};

pub type StepFn<'a, T> = Box<dyn Fn(T, &'a mut Graph) -> T + 'a>;

pub struct Traversal<'a, T: Display + IVertex> {
    graph: &'a Graph<'a>,
    steps: Vec<StepFn<'a, T>>,
}

impl<'a, T: Display + IVertex + 'a> Traversal<'a, T> {
    pub fn new(graph: &'a Graph) -> Traversal<'a, T> {
        Traversal {
            graph,
            steps: Vec::new(),
        }
    }

    fn add_step<F>(&mut self, step: F)
    where
        F: Fn(T, &'a mut Graph) -> T + 'a,
    {
        self.steps.push(Box::new(step));
    }

    pub fn v(mut self) -> Self {
        self
    }

    pub fn e(mut self) -> Self {
        self
    }

    pub fn in_v(mut self) -> Self {
        self
    }

    pub fn in_e(mut self) -> Self {
        self
    }

    pub fn out_v(mut self) -> Self {
        self
    }

    pub fn out_e(mut self) -> Self {
        self
    }

    pub fn drop(mut self) -> Self {
        self
    }

    pub fn add_v(mut self) -> Self {
        self
    }

    pub fn add_e(mut self) -> Self {
        self.add_step(VertexList::add_edge("test"));
        self
    }

    pub fn from(mut self) -> Self {
        self
    }

    pub fn to(mut self) -> Self {
        self
    }

    pub fn has(mut self) -> Self {
        self
    }

    pub fn property(mut self) -> Self {
        self
    }

    pub fn next(&mut self, res: T) -> T {
        self.steps
            .iter()
            .fold(res, |res, step| step(res, &mut self.graph))
    }
}

// impl<T: Display> Traversal<T> {
//     pub fn new(res: T) -> Traversal<T> {
//         Traversal { res }
//     }
// }

// impl<T: Display> std::ops::Deref for Traversal<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.res
//     }
// }
// impl<T: Display> std::ops::DerefMut for Traversal<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.res
//     }
// }

// impl<T: Display> Display for Traversal<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.res)
//     }
// }
