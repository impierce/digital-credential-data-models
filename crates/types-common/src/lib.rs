use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectOrVector<T> {
    Object(T),
    Vector(Vec<T>),
}

/// Very large structs use box construct to prevent too much memory from being copied.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BoxObjectOrVector<T> {
    Object(Box<T>),
    Vector(Vec<T>),
}
