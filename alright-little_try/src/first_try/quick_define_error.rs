use alright::{
    traits::{
        ExceptionUtils,
        Transform,
        TemplateDisplay
    },
    types::property::Property,
};
use alright_common::{
    PromiseErr,
};
use alright_derive::Exception;
use serde::Serialize;

// Using all above imports
// and add the derive macro `Exception` to your struct
#[derive(Exception, Debug, Clone, Serialize, Default)]
pub struct AnotherMyError {
    pub property: Box<Property<AnotherMyError>>,
}
