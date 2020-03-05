use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
  Symbol(Symbol),
  Number(f64),
  List(List),
}
pub type Scope = HashMap<String, Value>;
pub type Symbol = String;
pub type List = Vec<Value>;
