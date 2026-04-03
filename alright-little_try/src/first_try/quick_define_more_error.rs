use alright::{
    exceptions,
    Property,
    traits::{
        ExceptionUtils,
        TemplateDisplay,
        Transform,
        PromiseErr
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
