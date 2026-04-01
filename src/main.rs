use exception::exception::Exception;
use exception::property::Property;
use exception::traits::{ExceptionUtils, Transform};
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;

fn main() {
    #[derive(Debug, Clone, Serialize, Default)]
    struct ValueError {
        property: Box<Property<ValueError>>,
    }

    impl Transform<ValueError> for ValueError {
    }

    impl ExceptionUtils<ValueError> for ValueError {
        fn get_property(&self) -> Box<Property<ValueError>> {
            self.property.clone()
        }
        fn set_property(&mut self, property: Box<Property<ValueError>>) {
            self.property = property;
        }
        fn get_ptr(&self) -> ValueError {
            self.clone()
        }
        fn set_ptr(&mut self, ptr: ValueError) {
            *self = ptr;
        }
    }

    impl Error for ValueError {}
    impl Display for ValueError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.property.name)
        }
    }
    let value_err = ValueError {
        property: Box::new(
            Property {
                name: "ValueError".into(),
                context: vec!["this".to_string(), "is".to_string(), "a".to_string(), "test".to_string()],
                cause: None,
                other: Default::default(),
            }
        )
    };


    eprintln!("{:#?}\n", value_err);

    let exception: Exception<ValueError> = Exception::down(value_err.clone());
    eprintln!("{:#?}\n", exception);

    let upgrade: ValueError = Exception::up(exception);
    eprintln!("{:#?}\n", upgrade);
}
