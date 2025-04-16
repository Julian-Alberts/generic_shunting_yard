# Generic Shunting Yard

[![Crates.io](https://img.shields.io/crates/v/gyard?style=for-the-badge)](https://crates.io/crates/gyard)
![Crates.io](https://img.shields.io/crates/l/gyard?style=for-the-badge)
![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/Julian-Alberts/generic_shunting_yard/rust-test.yml?branch=main&label=Tests&style=for-the-badge)

The gyard crate is a generic implementation of the shunting yard algorythm with support for funtions. Not more not less.

 ```rust
 use gyard::{InputToken, OutputToken, op::Math, to_postfix};
 // 5 + 2 * sin(123)
 let infix = [
     InputToken::Value(5),
     InputToken::Operator(Math::Add),
     InputToken::Value(2),
     InputToken::Operator(Math::Mul),
     InputToken::Function("sin"),
     InputToken::LeftParen,
     InputToken::Value(123),
     InputToken::RightParen,
 ];
 let postfix = to_postfix(infix);
 assert_eq!(postfix, Ok(vec![
     OutputToken::Value(5),
     OutputToken::Value(2),
     OutputToken::Value(123),
     OutputToken::Function("sin"),
     OutputToken::Operator(Math::Mul),
     OutputToken::Operator(Math::Add),
 ]));
 ```

You can define your own operators using the `gyard::Operator` trait.
```rust
pub struct MyOp;
impl gyard::Operator for MyOp {
    fn precedence(&self) -> usize {
        10
    }
    fn is_left_associative(&self) -> bool {
        true
    }
}
```
The default operator implementations use the precedence defined in JavaScript.

Values and functions do not require any special traits.

