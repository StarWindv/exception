use crate::traits::Transform;
use crate::Property;

pub trait ExceptionUtils<T: Transform> {
    fn get_property (&self) -> Box<Property<T>>;
    fn set_property (&mut self, property: Box<Property<T>>);
    fn get_ptr (&self) -> T;
    fn set_ptr (&mut self, ptr: T);
}