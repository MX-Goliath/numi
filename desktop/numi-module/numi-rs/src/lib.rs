#![allow(dead_code, clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

mod app;
mod attr_text;
mod calc;
mod calc_functions;
mod calc_ops;
mod calc_scales;
mod calc_units;
mod calc_vars;
mod currency;
mod date;
mod element;
mod element_list;
mod element_list_builder;
mod eval;
mod eval_context;
mod extr;
mod fns;
mod formatter;
mod localizer;
mod matcher;
mod op;
mod phrase_detector;
mod remote_manager;
mod scale;
mod si;
mod srange;
mod str_ext;
mod str_ranger;
mod strings_parser;
mod substr;
mod temperature;
mod tz;
mod unit_converter;
mod unit_manager;
mod utils;
mod value;
mod wave;

pub use app::NumiModel;
pub use app::NumiModelOptions;
pub use calc::Calc;
pub use calc::CalcConfig;
pub use calc::CalcInput;
pub use calc::CalcOutput;
pub use calc::CalcResult;
pub use currency::*;
pub use date::DateTime;
pub use formatter::{FormatOptions, Formatter};
pub use remote_manager::*;
pub use unit_manager::Unit;
pub use value::Value;

#[allow(unused_imports)]
pub(crate) use attr_text::{AttrEl, AttrKind, AttrText};
pub(crate) use element::Element;
pub(crate) use element_list::ElementList;
pub(crate) use eval::Evaluator;
pub(crate) use eval_context::EvalContext;
pub(crate) use localizer::Localizer;
pub(crate) use matcher::{Match, Matcher};
pub(crate) use srange::SRange;
pub(crate) use str_ranger::StrRanger;
pub(crate) use substr::Substr;
pub(crate) use unit_manager::UnitManager;

#[no_mangle]
pub extern "C" fn test() {
  println!("Hello from Rust! ❤️");
}

#[no_mangle]
pub extern "C" fn create_calc() -> *mut c_void {
  let calc = Calc::new();
  let b = Box::new(calc);
  Box::into_raw(b) as *mut c_void
}

#[no_mangle]
pub extern "C" fn free_calc(calc: *mut c_void) {
  unsafe {
    _ = Box::from_raw(calc as *mut Calc);
  }
}

#[no_mangle]
pub extern "C" fn calc_json(calc: *mut c_void, text: *const c_char) -> *mut c_char {
  unsafe {
    let c_str = CStr::from_ptr(text);
    let calc_ref = (calc as *mut Calc).as_ref().unwrap();
    let result = calc_ref.calc_json(c_str.to_str().unwrap());
    CString::new(result).unwrap().into_raw()
  }
}

#[no_mangle]
pub extern "C" fn calc_calculate(calc: *mut c_void, text: *const c_char) -> *mut c_char {
  unsafe {
    let c_str = CStr::from_ptr(text);
    let format_options = FormatOptions::default();
    let calc_ref = (calc as *mut Calc).as_ref().unwrap();
    let result = calc_ref.calculate_format(c_str.to_str().unwrap(), &format_options);
    CString::new(result.unwrap_or_default()).unwrap().into_raw()
  }
}

#[no_mangle]
pub extern "C" fn calc_fill_rates(calc: *mut c_void, rates_text: *const c_char) {
  unsafe {
    let c_str = CStr::from_ptr(rates_text);
    let calc_ref = (calc as *mut Calc).as_mut().unwrap();
    calc_ref.fill_rates_from_json_str(c_str.to_str().unwrap());
  }
}

#[no_mangle]
pub extern "C" fn calc_free_result(s: *mut c_char) {
  unsafe {
    if s.is_null() {
      return;
    }
    let _ = CString::from_raw(s);
  };
}

#[no_mangle]
pub extern "C" fn test_calculate(text: *const c_char) -> *mut c_char {
  let c_str = unsafe { CStr::from_ptr(text) };
  let len = c_str.to_bytes().len();
  let result = format!("R{len}");
  CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn test_calculate_free(s: *mut c_char) {
  unsafe {
    if s.is_null() {
      return;
    }
    let _ = CString::from_raw(s);
  };
}
