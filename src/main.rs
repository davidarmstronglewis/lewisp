#![allow(unused)]

mod lib;
use lib::{environment::*, types::*};

use std::collections::HashMap;
use std::io::{self, prelude::*, StderrLock, StdinLock, StdoutLock};

#[derive(Debug, PartialEq)]
pub enum Error {}

pub fn tokenize(program: &str) -> Vec<String> {
  program
    .replace("(", " ( ")
    .replace(")", " ) ")
    .split_whitespace()
    .map(str::to_string)
    .collect()
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Value, &'a [String]), Error> {
  unimplemented!()
}

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let stdout = io::stdout();
  let stderr = io::stderr();

  {
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();
    let mut input_buf = String::new();
    loop {
      stdout.write(b" lisp > ");
      stdout.flush();

      stdin.read_line(&mut input_buf)?;

      process_input(&input_buf);
      input_buf.clear();
    }
  }
}

fn process_input(input: &str) {
  match input {
    "" => std::process::exit(0),
    _ => {
      let parsed = tokenize(input);
      println!("{:#?}", parsed);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn environment_search() {
    let mut env = Environment::init();
    let var_name = "test";

    assert_eq!(env.get(&var_name), None);

    env.push_scope();
    env.set(Symbol::from(var_name), Value::Number(42.0));
    assert_eq!(env.get(&var_name), Some(&Value::Number(42.0)));

    env.push_scope();
    assert_eq!(env.get(&var_name), Some(&Value::Number(42.0)));

    env.pop_scope();
    assert_eq!(env.get(&var_name), Some(&Value::Number(42.0)));

    env.pop_scope();
    assert_eq!(env.get(&var_name), None);
  }

  #[test]
  fn basic_addition() {
    let expr = "(+ 137 349)";
    let tokens = tokenize(&expr);

    assert_eq!(tokens, vec!["(", "+", "137", "349", ")"]);

    let inner_tokens: Vec<String> = vec!["137".into(), "349".into()];
    let parsed = parse(&tokens).unwrap();

    assert_eq!(
      parsed,
      (Value::Symbol(Symbol::from("+")), inner_tokens.as_ref())
    );
  }

  #[test]
  fn basic_expressions() {
    let expr1 = "(+ 137 349)";
    let expr2 = "(- 1000 334)";
    let expr3 = "(* 5 99)";
    let expr4 = "(/ 10 5)";
    let expr5 = "(+ 2.7 10)";
  }

  #[test]
  fn tree_representation() {
    let expr = r#"
    (*
      (+ 2
        (* 4 6))
      (+ 3 5 7))
    "#;
  }

  #[test]
  fn procedure_definition() {
    // The general form of a procedure is
    // (define (<name> <formal parameters>) <body>)
    let expr = r#"
      (define (square x) (* x x))
    "#;
  }
}
