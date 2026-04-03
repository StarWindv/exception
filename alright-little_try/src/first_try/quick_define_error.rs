use alright::{
    traits::{
        ExceptionUtils,
        Transform,
        TemplateDisplay,
        PromiseErr,
    },
    Property,
};
use alright::Exception;
use serde::Serialize;

// Using all above imports
// and add the derive macro `Exception` to your struct
#[derive(Exception, Debug, Clone, Serialize)]
pub struct AnotherMyError {
    pub property: Box<Property<AnotherMyError>>,
}
