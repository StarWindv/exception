use serde::Serialize;
use serde_json::{Map, Value};
use crate::exception::Exception;
use crate::traits::Transform;

// property.rs
#[derive(Debug, Serialize, Clone)]
pub struct Property<T: Transform<T>> {   // 加上约束
    pub name: String,
    pub context: Vec<String>,
    pub cause: Option<Exception<T>>,
    pub other: Map<String, Value>
}

impl<T: Transform<T>> Default for Property<T> {
    fn default() -> Self {
        Self {
            name: String::new(),
            context: Vec::new(),
            cause: None,
            other: Map::new(),
        }
    }
}
