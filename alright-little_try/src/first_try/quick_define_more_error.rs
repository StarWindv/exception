use alright_common::PromiseErr;
use alright::{
    exceptions,
    types::property::Property,
    traits::{
        ExceptionUtils,
        TemplateDisplay,
        Transform
    },
};
// Using these

// And then
// you can use the macro `exceptions!`
// to define more and more Errors
exceptions!(
    MyError1,
    MyError2,
    YetAnotherError
);
