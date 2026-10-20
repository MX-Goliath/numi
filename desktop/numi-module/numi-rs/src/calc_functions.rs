use std::rc::Rc;

use crate::{
  date::DateFns,
  fns::{CommonFns, TrigonometryFns},
  localizer::Localizer,
  Calc,
};

impl Calc {
  pub fn add_functions(&self, _localizer: &Localizer) {
    let common_fns = Rc::<CommonFns>::default();
    self.add_func("sqrt", &vec!["sqrt"], common_fns.clone());
    self.add_func("cbrt", &vec!["cbrt"], common_fns.clone());
    self.add_func("log", &vec!["log"], common_fns.clone());
    self.add_func("ln", &vec!["ln"], common_fns.clone());
    self.add_func("round", &vec!["round"], common_fns.clone());
    self.add_func("ceil", &vec!["ceil"], common_fns.clone());
    self.add_func("floor", &vec!["floor"], common_fns.clone());
    self.add_func("abs", &vec!["abs"], common_fns);

    let trig_fns = Rc::<TrigonometryFns>::default();
    self.add_func("sin", &vec!["sin"], trig_fns.clone());
    self.add_func("asin", &vec!["asin"], trig_fns.clone());
    self.add_func("sinh", &vec!["sinh"], trig_fns.clone());
    self.add_func("cos", &vec!["cos"], trig_fns.clone());
    self.add_func("acos", &vec!["acos"], trig_fns.clone());
    self.add_func("cosh", &vec!["cosh"], trig_fns.clone());
    self.add_func("tan", &vec!["tan"], trig_fns.clone());
    self.add_func("atan", &vec!["atan"], trig_fns.clone());
    self.add_func("tanh", &vec!["tanh"], trig_fns);

    let date_fns = Rc::<DateFns>::default();
    self.add_func("now", &vec!["now", "time"], date_fns.clone());
    self.add_func("today", &vec!["today"], date_fns.clone());
    self.add_func("tomorrow", &vec!["tomorrow"], date_fns.clone());
    self.add_func("yesterday", &vec!["yesterday"], date_fns);
  }
}
