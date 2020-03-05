#![allow(unused)]

use std::collections::HashMap;
use std::io::{self, prelude::*, StderrLock, StdinLock, StdoutLock};

#[derive(Debug, PartialEq)]
pub enum Error {}

pub type Scope = HashMap<String, Value>;
pub type Symbol = String;
pub type List = Vec<Value>;

#[derive(Debug, PartialEq)]
pub enum Value {
  Symbol(Symbol),
  Number(f64),
  List(List),
}

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

pub struct Environment {
  scopes: Vec<Scope>,
}
impl Environment {
  pub fn init() -> Environment {
    let mut scopes = Vec::new();
    scopes.push(Scope::new());
    Environment { scopes }
  }

  pub fn push_scope(&mut self) {
    self.scopes.push(Scope::new());
  }

  pub fn pop_scope(&mut self) {
    self.scopes.pop();
  }

  pub fn get(&self, symbol: &str) -> Option<&Value> {
    self
      .scopes
      .iter()
      .rev()
      .find(|scope| scope.contains_key(symbol))
      .and_then(|scope| scope.get(symbol))
  }

  pub fn set(&mut self, symbol: String, value: Value) {
    self
      .scopes
      .iter_mut()
      .last()
      .and_then(|mut scope| scope.insert(symbol, value));
  }
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

      stdin.read_line(&mut input_buf);
      process_input(&input_buf, &mut stdin, &mut stdout, &mut stderr);

      input_buf.clear();
    }
  }
}

fn process_input(
  input: &str,
  stdin: &mut StdinLock,
  stdout: &mut StdoutLock,
  stderr: &mut StderrLock,
) {
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
    let parsed = parse(&tokens);
    assert_eq!(
      parsed,
      Ok((Value::Symbol(Symbol::from("+")), inner_tokens.into()))
    )
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
