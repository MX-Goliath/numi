mod abs_fn;
mod common_fns;
mod fn_evaluator;
mod fn_extractor;
mod fn_matcher;
mod fn_registry;
mod trigonometry_fns;

#[allow(unused_imports)]
pub use abs_fn::*;
pub use common_fns::*;
pub use fn_evaluator::*;
pub use fn_extractor::*;
pub use fn_matcher::*;
pub use fn_registry::*;
pub use trigonometry_fns::*;

use crate::{eval_context::DataContext, Value};

pub type FuncId = String;

pub trait Func {
  fn evaluate(&self, fn_id: &str, values: &[Value], context: &DataContext) -> Option<Value>;
}
