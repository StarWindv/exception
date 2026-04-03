use crate::modules::types::{
    exception::BaseException,
    property::Property
};
use serde::Serialize;
use std::fmt::{Debug, Formatter};

pub trait ExceptionUtils<T: Transform> {
    fn get_property (&self) -> Box<Property<T>>;
    fn set_property (&mut self, property: Box<Property<T>>);
    fn get_ptr (&self) -> T;
    fn set_ptr (&mut self, ptr: T);
}

pub trait Transform: Clone + Debug + Serialize + 'static {
    fn down(&self) -> BaseException<Self>
    where
        Self: Transform + ExceptionUtils<Self>,
    {
        let mut inner = self.clone();
        inner.set_property(Box::new(Property::<Self> {
            name: "".to_string(),
            ..Default::default()
        }));
        BaseException {
            property: self.get_property(),
            target_ptr: inner,
        }
    }

    fn up(this: BaseException<Self>) -> Self
    where
        Self: ExceptionUtils<Self> + 'static + std::error::Error + Serialize + Clone,
    {
        let mut result: Self = this.get_ptr();
        result.set_property(this.property.clone());
        result
    }
}

pub trait TemplateDisplay<T: Transform>: ExceptionUtils<T> {
    fn template_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::ser::to_string_pretty(&self.get_property()) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => {
                let downgrade = format!(
                    "{}\n{}\n{}\n{}",
                    format!("name :  {}", self.get_property().name),
                    format!(
                        "cause:  {:?}",
                        match &self.get_property().cause {
                            Some(cause) => write!(f, "{}", cause),
                            None => write!(f, "None"),
                        }
                    ),
                    format!("context:{}", self.get_property().context.join("\n - ")),
                    format!("other:  {:#?}", self.get_property().other),
                );
                write!(f, "{}", downgrade)
            }
        }
    }
}
