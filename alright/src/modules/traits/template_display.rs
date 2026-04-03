use std::fmt::Formatter;
use crate::traits::ExceptionUtils;
use crate::traits::Transform;

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
