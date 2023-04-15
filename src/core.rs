pub struct Classes {
    them: Vec<Class>,
}

impl Classes {
    pub fn new() -> Self {
        Self { them: Vec::new() }
    }

    pub fn add(&mut self, class: Class) -> &mut Self {
        self.them.push(class);
        self
    }

    pub fn collect(&self) -> String {
        self.them
            .iter()
            .filter_map(|class| class.get())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Default for Classes {
    fn default() -> Self {
        Classes::new()
    }
}

pub struct Class(Option<String>);

impl Class {
    pub fn get(&self) -> Option<String> {
        self.0.clone()
    }

    pub fn new(value: Option<String>) -> Self {
        Self(value.and_then(|it| if it.is_empty() { None } else { Some(it) }))
    }
}

impl From<String> for Class {
    fn from(value: String) -> Self {
        Class::new(Some(value))
    }
}

impl From<&str> for Class {
    fn from(value: &str) -> Self {
        Class::new(Some(value.into()))
    }
}

impl From<Option<String>> for Class {
    fn from(value: Option<String>) -> Self {
        Class::new(value)
    }
}

impl From<Option<&str>> for Class {
    fn from(value: Option<&str>) -> Self {
        Class::new(value.map(|it| it.into()))
    }
}

#[macro_export]
macro_rules! classes {
    ($($token:expr$(=> $bool:expr)?),*) => [
        {
            let mut classes = Classes::new();

            $(
                if true $(&& $bool)? {
                    classes.add($token.into());
                }
            )*

            classes.collect()
        }
    ];
}

#[cfg(test)]
mod tests {
    use crate::core::*;

    macro_rules! tests {
        [$([$test_name:ident, $actual:expr, $expected:literal]),+$(,)?] => {
            $(
                #[test]
                fn $test_name() {
                    assert_eq!($actual, $expected);
                }
            )+
        }
    }

    const DISABLED: bool = true;

    tests![
        [should_accept_str_and_strings, classes!["button".to_string(), "button--disabled"], "button button--disabled"],
        [should_accept_optionals, classes![Some("button--active"), None::<String>, Some("button--disabled".to_string())], "button--active button--disabled"],
        [should_accept_expressions, classes!["concatenated".to_string() + "-class", Some("batman").map(|_| "bruce-wayne")], "concatenated-class bruce-wayne"],
        [should_apply_classes_evaluating_to_true, classes!["button" => true, "button--disabled" => DISABLED, "button--active" => false, "all-the-buttons" => 42 > 3 ], "button button--disabled all-the-buttons"],
        [should_accept_various_types_at_the_same_time, classes!["button" => true, Some("button--disabled"), None::<String>, "button--primary"], "button button--disabled button--primary"],
        [should_remove_empty_str, classes!["button", "", "button--active"], "button button--active"],
        [should_remove_empty_string, classes!["button", "".to_string(), "button--active"], "button button--active"],
        [should_remove_empty_strings_passed_as_options, classes!["button", Some(""), "button--active"], "button button--active"],
    ];
}