[![Checks](https://github.com/sparten11740/classes/actions/workflows/ci.yml/badge.svg)](https://github.com/sparten11740/classes/actions/workflows/ci.yml)
# classes

`classes` is a lightweight and dependency-free macro that simplifies the process of building class strings for DOM
elements. It accepts a variable number of arguments and combines them into a single class string.
This macro is designed after the popular `classnames` npm package, which is commonly used in React and other frameworks.

## Usage

You can supply string types or types that can be transformed into a string to the macro:

- `Option<String>` / `Option<&str>` will use the inner value if the option is `Some`, and ignore the option if
  it's `None`
- `String` / `&str` will be applied as is
- The special syntax `string_expr => bool_expr` will use the `string_expr` when `bool_expr` evaluates to true

Using the Classes macro can simplify your code by reducing the boilerplate needed to build class strings.

### Example

```rust
use classes::classes;

fn main() {
    let optional = Some("lumos");
    let is_night = true;

    let class = classes!["hogwarts", optional, "hogwarts--at-night" => is_night, "wingardium-leviosa" => false];

    println!("{class}"); // => 'hogwarts lumos hogwarts--at-night'
}

```

### Dioxus Example

```rust
use classes::classes;

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let class = classes!["button", "button--disabled" => cx.props.disabled, cx.props.class];

    cx.render(rsx! {
      button { class }
    })
}

```