use std::error::Error;
use std::fmt::Display;
use serde::Serialize;
use alright::{
    traits::{
        ExceptionUtils,
        TemplateDisplay,
        Transform,
    },
    types::property::Property
};
use alright_common::PromiseErr;

#[derive(Debug, Clone, Serialize, Default)]
pub struct MyError {
    // Required field
    // name doesn't matter
    // Of cause, you can also use type alias
    // like: `use alright::types::property::Property as Prop;`
    pub property: Box<Property<MyError>>,
}

impl Transform for MyError {}  // Implement these empty traits
impl Error for MyError {}      // [ using default implementations ]

// If you need a general type to encapsulate all errors
// then implement this empty trait
// warning:
//  - If you use this general type ( `AlrightBox` )
//  - then it will be difficult for you to obtain the actual type.
impl PromiseErr for MyError {}

impl ExceptionUtils<MyError> for MyError {
    fn get_property(&self) -> Box<Property<MyError>> {
        self.property.clone()
    }
    fn set_property(&mut self, property: Box<Property<MyError>>) {
        self.property = property;
    }
    fn get_ptr(&self) -> MyError {
        self.clone()
    }
    fn set_ptr(&mut self, ptr: MyError) {
        *self = ptr;
    }
}

// Using template
impl TemplateDisplay<MyError> for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.template_fmt(f)
    }
}
