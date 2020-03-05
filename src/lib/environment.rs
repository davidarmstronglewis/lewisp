use crate::lib::types::{Scope, Value};

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
