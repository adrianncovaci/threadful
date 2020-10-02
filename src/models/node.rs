use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<T> {
    data: T,
    childs: Vec<Node<T>>,
}
