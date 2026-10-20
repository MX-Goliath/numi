mod binary_op;
mod decimal_op;
mod decimal_op_eval;
mod decimal_op_matcher;
mod op_extractor;
mod op_matcher;
mod op_registry;
mod split_op_eval;
mod split_op_matcher;

pub use binary_op::*;
pub use decimal_op::*;
pub use decimal_op_eval::*;
pub use decimal_op_matcher::*;
pub use op_extractor::*;
pub use op_matcher::*;
pub use op_registry::*;
pub use split_op_eval::*;
pub use split_op_matcher::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Op {
  pub id: String,
}

impl Op {
  pub fn new(id: &str) -> Self {
    Self { id: id.to_string() }
  }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum OpType {
  Any,
  Some(String),
}

impl OpType {
  pub fn with_some(str: &str) -> Self {
    OpType::Some(str.to_string())
  }
}
