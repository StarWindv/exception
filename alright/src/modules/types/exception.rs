use serde::Serialize;

use super::{
    property::Property,
    super::traits::Transform
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct BaseException<T: Transform> {
    pub property: Box<Property<T>>,
    pub target_ptr: T,
}
