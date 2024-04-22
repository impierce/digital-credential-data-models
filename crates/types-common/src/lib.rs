use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectOrVector<T> {
    Object(T),
    Vector(Vec<T>),
}

impl<T: Clone> From<&ObjectOrVector<T>> for ObjectOrVector<T> {
    fn from(value: &ObjectOrVector<T>) -> Self {
        value.clone()
    }
}

impl<T> From<T> for ObjectOrVector<T> {
    fn from(value: T) -> Self {
        Self::Object(value)
    }
}

impl<T> From<Vec<T>> for ObjectOrVector<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Vector(value)
    }
}

/// Very large structs use box construct to prevent too much memory from being copied.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BoxObjectOrVector<T> {
    Object(T),
    Vector(Vec<T>),
}

impl<T: Clone> From<&BoxObjectOrVector<T>> for BoxObjectOrVector<T> {
    fn from(value: &BoxObjectOrVector<T>) -> Self {
        value.clone()
    }
}

impl<T> From<T> for BoxObjectOrVector<T> {
    fn from(value: T) -> Self {
        Self::Object(value)
    }
}

impl<T> From<Vec<T>> for BoxObjectOrVector<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Vector(value)
    }
}
