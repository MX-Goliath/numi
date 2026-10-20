mod assignment_eval;
mod bin_extractor;
mod combiner;
mod converter_eval;
mod dec_unit_seq;
mod decimal_unit;
mod first_value;
mod hex_extractor;
mod list_eval;
mod number_extractor;
mod oct_extractor;
mod parentheses;
mod percent;
mod regex_extractor;
mod repr_eval;
mod repr_extractor;
mod sci_number_extractor;
mod signed_number;
mod split_eval;
mod str_filler;
mod str_infuser;
mod token_eval;
mod unit_extractor;
mod var_extractor;

pub use assignment_eval::*;
pub use bin_extractor::*;
pub use combiner::*;
pub use converter_eval::*;
pub use dec_unit_seq::*;
pub use decimal_unit::*;
pub use first_value::*;
pub use hex_extractor::*;
pub use list_eval::*;
pub use number_extractor::*;
pub use oct_extractor::*;
pub use parentheses::*;
pub use percent::*;
pub use regex_extractor::*;
pub use repr_eval::*;
pub use repr_extractor::*;
pub use sci_number_extractor::*;
pub use signed_number::*;
pub use split_eval::*;
pub use str_filler::*;
pub use str_infuser::*;
pub use token_eval::*;
pub use unit_extractor::*;
pub use var_extractor::*;

use super::element_list::*;
use crate::EvalContext;
use std::ops::Deref;

pub trait Evaluator {
  /// Evaluates elements sequence, should return empty list if can't be evaluated
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>>;

  fn boxed(self) -> Box<Self>
  where
    Self: Sized,
  {
    Box::new(self)
  }

  fn type_name(&self) -> &str {
    let type_name = std::any::type_name::<Self>();
    let parts: Vec<&str> = type_name.split("::").collect();
    let struct_name = parts.last().unwrap_or(&"Unknown");
    struct_name
  }
}

// May no longer be required
impl<T, U> Evaluator for T
where
  T: Deref<Target = U>,
  U: Evaluator,
{
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    return self.deref().evaluate(elements, context);
  }
}
