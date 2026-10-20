use crate::currency::CurrencyConverter;
use crate::date::DatetimeExtractor;
use crate::eval::*;
use crate::eval_context::DataContext;
use crate::fns::Func;
use crate::fns::FuncExtractor;
use crate::fns::FuncManager;
use crate::formatter::FormatOptions;
use crate::op::DecimalOp;
use crate::op::OpRegistry;
use crate::scale::{ScaleEvaluator, ScaleExtractor, ScaleManager};
use crate::tz::TzConverter;
use crate::tz::TzDateEval;
use crate::tz::TzExtractor;
use crate::unit_converter::{RatioValueConverter, UnitConverter, ValueConverter};
use crate::unit_manager::Unit;
use crate::wave::WaveConverter;
use crate::*;
use date::DateOp;
use date::DateOpEval;
use extr::DateExtractor;
use fns::FuncEvaluator;
use log::debug;
use op::{DecimalOpEvaluator, OpExtractor, SplitOpEvaluator};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Calc {
  pub localizer: Localizer,
  pub unit_manager: Rc<RefCell<UnitManager>>,
  pub unit_converter: Rc<RefCell<UnitConverter>>,
  pub ratio_converter: Rc<RefCell<RatioValueConverter>>,
  pub func_manager: Rc<RefCell<FuncManager>>,
  pub scale_manager: Rc<RefCell<ScaleManager>>,
  pub decimal_op: Rc<RefCell<OpRegistry>>,
  pub data_context: DataContext,
  parser: ListEval,
  eval: ListEval,
  formatter: Formatter,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CalcConfig {
  pub locale_path: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CalcInput {
  pub input: Vec<String>,
  pub options: Option<FormatOptions>,
}

impl CalcInput {
  pub fn with_lines(lines: Vec<&str>) -> Self {
    let input = lines.iter().map(|l| l.to_string()).collect();
    Self {
      input,
      options: None,
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CalcOutput {
  pub result: Vec<CalcResult>,
}

impl CalcOutput {
  pub fn new(result: Vec<CalcResult>) -> Self {
    Self { result }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CalcResult {
  pub value: Option<String>,
  pub error: Option<String>,
  pub text: Option<Vec<CalcAttrTextEl>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CalcAttrTextEl {
  pub string: String,
  pub stype: String,
}

impl CalcAttrTextEl {
  pub fn new(string: String, stype: String) -> Self {
    Self { string, stype }
  }
}

unsafe impl Send for Calc {}

impl Calc {
  pub fn new() -> Self {
    Self::with_config(&CalcConfig::default())
  }

  pub fn with_config(config: &CalcConfig) -> Calc {
    let mut localizer = Localizer::new();

    debug!("Initialize calc with config: {:?}", config);
    localizer.load_en_strings();
    if let Some(locale_path) = &config.locale_path {
      localizer.parse_loc_file(locale_path);
    }
    let unit_manager = Rc::new(RefCell::new(UnitManager::default()));
    let ratio_conv = Rc::new(RefCell::new(RatioValueConverter::default()));
    let mut unit_converter = UnitConverter::default();
    unit_converter.add_converter(Box::new(WaveConverter::default()));
    unit_converter.add_converter(Box::new(ratio_conv.clone()));
    let currency_conv = CurrencyConverter::default();
    unit_converter.add_converter(Box::new(currency_conv));
    unit_converter.add_converter(Box::new(TzConverter::default()));
    let rc_converter = Rc::new(RefCell::new(unit_converter));

    let decimal_op = Rc::new(RefCell::new(OpRegistry::default()));
    let func_manager = Rc::new(RefCell::new(FuncManager::default()));
    let scale_manager = Rc::new(RefCell::new(ScaleManager::default()));

    let range_detector = Rc::new(StrRanger::new());

    // Extractors

    let mut repr_extractor = ReprExtractor::default();
    repr_extractor.add_repr("sci", &localizer.get_strings("scientific.variants"));
    repr_extractor.add_repr("bin", &localizer.get_strings("binary.variants"));
    repr_extractor.add_repr("hex", &localizer.get_strings("hex.variants"));
    repr_extractor.add_repr("oct", &localizer.get_strings("octal.variants"));
    repr_extractor.add_repr("dec", &localizer.get_strings("decimal.variants"));

    let op_extractor = OpExtractor::new(decimal_op.clone());
    let number_extractor = NumberExtractor::new(range_detector.clone());
    let sci_number_extractor = SciNumberExtractor::new(range_detector.clone());
    let unit_extractor = UnitExtractor::new(unit_manager.clone());
    let func_extractor = FuncExtractor::new(func_manager.clone());
    let scale_extractor = ScaleExtractor::new(scale_manager.clone());
    let parser = Combiner::new(vec![
      DateExtractor::new(range_detector.clone()).boxed(),
      DatetimeExtractor::new(range_detector.clone()).boxed(),
      TzExtractor::default().boxed(),
      sci_number_extractor.boxed(),
      HexExtractor::new(range_detector.clone()).boxed(),
      OctExtractor::new(range_detector.clone()).boxed(),
      BinExtractor::new(range_detector.clone()).boxed(),
      number_extractor.boxed(),
      repr_extractor.boxed(),
      op_extractor.boxed(),
      scale_extractor.boxed(),
      unit_extractor.boxed(),
      func_extractor.boxed(),
      VarExtractor::default().boxed(),
    ]);

    let parse_eval = ListEval::with_evaluators(vec![Box::new(parser), Box::new(StrFiller::new())]);

    // Evaluator
    let dec_binary_op = Rc::new(DecimalOp::new(rc_converter.clone()));
    let percent_binary_op = Rc::new(PercentBinaryOp::new(rc_converter.clone()));

    let split_op_eval = SplitOpEvaluator::new(decimal_op.clone());
    let rec_eval = ListEval::with_evaluators(vec![
      AssignmentEval::default().boxed(),
      Box::new(StrInfuser::new()), // TODO: potentially can be move to parser, assignment eval should be adjusted
      ParenthesesEvaluator::new().boxed(),
      DecimalUnitEvaluator::new().boxed(),
      split_op_eval.boxed(),
      ScaleEvaluator::new(scale_manager.clone()).boxed(),
      DecUnitSequenceEval::new(rc_converter.clone()).boxed(),
      DecimalUnitEvaluator::new().boxed(),
      DecimalOpEvaluator::new(dec_binary_op.clone()).boxed(),
      DateOpEval::new(Rc::new(DateOp::new(rc_converter.clone()))).boxed(),
      PercentEval::new(percent_binary_op.clone()).boxed(),
      SignedNumberEval::default().boxed(),
      ReprConverterEval::default().boxed(),
      FuncEvaluator::new(func_manager.clone()).boxed(),
      ConverterEval::new(rc_converter.clone()).boxed(),
      TzDateEval::new(rc_converter.clone()).boxed(),
      FirstValue::new().boxed(),
      TokenEval::new(dec_binary_op.clone()).boxed(),
    ]);

    let formatter = Formatter::new(unit_manager.clone());
    let data_context = DataContext::default();

    let mut calc = Calc {
      parser: parse_eval,
      eval: rec_eval,
      unit_manager,
      unit_converter: rc_converter,
      ratio_converter: ratio_conv,
      func_manager,
      scale_manager,
      decimal_op,
      formatter,
      data_context,
      localizer,
    };

    calc.add_units(&calc.localizer);
    calc.add_functions(&calc.localizer);
    calc.add_scales(&calc.localizer);
    calc.add_temperature(&calc.localizer);
    calc.add_vars();
    calc.add_ops(&calc.localizer);
    calc
  }

  pub fn calculate_format(&self, str: &str, options: &FormatOptions) -> Option<String> {
    let element = self.calculate(str);
    self.formatter.format(&element.value, options)
  }

  pub fn calculate<'a>(&self, str: &'a str) -> Element<'a> {
    debug!("Calculating: {}", str);
    let el = Element::with_str(str);
    let mut result = ElementList::with_element(el.clone());
    let context = EvalContext::new(&self.eval, self.data_context.clone());
    if let Some(parse_result) = self.parser.evaluate(&result, &context) {
      result = parse_result;
    }
    debug!("Parsed element list: {:?}", &result);
    if let Some(eval_result) = self.eval.evaluate(&result, &context) {
      result = eval_result;
    }
    return result.first().unwrap_or(&el).clone();
  }

  pub fn calc_json(&self, input_str: &str) -> String {
    let input_opt: Result<CalcInput> = serde_json::from_str(input_str);
    let Ok(input) = input_opt else {
      let mut format_err: String = "unknown parse error".to_string();
      if let Err(err) = input_opt {
        format_err = format!("Parse error: {:?}", err);
        eprintln!("Error: {:?}", err);
      };
      let parse_error = CalcResult {
        value: None,
        error: Option::Some(format_err),
        text: None,
      };
      return serde_json::to_string(&parse_error).unwrap();
    };
    let output = self.calc(&input);
    serde_json::to_string(&output).unwrap()
  }

  pub fn calc(&self, input: &CalcInput) -> CalcOutput {
    let context = EvalContext::new(&self.eval, self.data_context.clone());
    let mut result: Vec<CalcResult> = Vec::new();
    let options = input.options.clone().unwrap_or_default();
    for line in input.input.iter() {
      let line_result = self.calculate_line(&options, &context, line);
      result.push(line_result);
    }
    CalcOutput { result }
  }

  pub fn calculate_line(
    &self,
    options: &FormatOptions,
    context: &EvalContext,
    str: &str,
  ) -> CalcResult {
    let el = Element::with_str(str);
    let mut result = ElementList::with_element(el.clone());
    if let Some(parse_result) = self.parser.evaluate(&result, context) {
      result = parse_result;
    }
    // dbg!(&result);
    if let Some(eval_result) = self.eval.evaluate(&result, context) {
      result = eval_result;
    }
    let first_el = result.first().unwrap_or(&el).clone();
    let format_res = self.formatter.format(&first_el.value, options);
    let attr_text = result.attr_text().unwrap();

    CalcResult {
      value: format_res,
      error: None,
      text: Some(self.export_attr_text(&attr_text)),
    }
  }

  // TODO: unit test
  fn export_attr_text(&self, atext: &AttrText) -> Vec<CalcAttrTextEl> {
    let mut res = Vec::new();

    let mut i_start = atext.substr.range().start;
    for el in &atext.markup {
      // Add previous string part without any attributes
      if el.range.start > i_start {
        let r = i_start..el.range.start;
        let s = Substr::new(atext.substr.whole_str(), r);
        let rel = CalcAttrTextEl::new(s.str_in_range().to_string(), "".to_string());
        res.push(rel);
      }
      let s = Substr::new(atext.substr.whole_str(), el.range);
      let rel = CalcAttrTextEl::new(s.str_in_range().to_string(), el.kind.to_string());
      res.push(rel);
      i_start = el.range.end();
    }
    if i_start < atext.substr.range().end() {
      let r = i_start..atext.substr.range().end();
      let s = Substr::new(atext.substr.whole_str(), r);
      let rel = CalcAttrTextEl::new(s.str_in_range().to_string(), "".to_string());
      res.push(rel);
    }
    res
  }

  pub fn add_unit(&self, unit: Unit, phrases: &Vec<&str>) {
    let mut um = self.unit_manager.borrow_mut();
    um.add_unit(unit, phrases)
  }

  pub fn add_converter(&self, conv: Box<dyn ValueConverter>) {
    let mut converter = self.unit_converter.borrow_mut();
    converter.add_converter(conv);
  }

  pub fn add_func(&self, id: &str, phrases: &Vec<&str>, func: Rc<dyn Func>) {
    let mut func_manager = self.func_manager.borrow_mut();
    func_manager.add_func(id, phrases, func);
  }

  pub fn set_context_value(&mut self, key: &str, value: &Value) {
    self.data_context.set_value(key, value)
  }
}

impl Default for Calc {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use pretty_assertions::assert_eq;
  use rust_decimal_macros::dec;

  #[test]
  fn test_calculate() {
    let calc = Calc::new();
    assert_eq!(calc.calculate(""), Element::with_str(""))
  }

  #[test]
  fn test_number() {
    let calc = Calc::new();
    let text = " 1 ";
    assert_eq!(
      calc.calculate(text),
      Element::with_decimal_range_in_str(dec!(1), 1..2, text)
    );
  }

  #[test]
  fn test_syntax_highlight() {
    let calc = Calc::new();
    let text = "1 + 2%";

    assert_eq!(
      calc.calculate(text).attr_text,
      AttrText::new(
        Substr::with_str(text),
        vec![
          AttrEl::new(SRange::from(0..1), AttrKind::Number),
          AttrEl::new(SRange::from(2..3), AttrKind::Op),
          AttrEl::new(SRange::from(4..5), AttrKind::Number),
          AttrEl::new(SRange::from(5..6), AttrKind::Unit),
        ],
      ),
    )
  }

  #[test]
  fn test_calc_json() {
    let calc = Calc::new();
    let input = r#"{"input":["1+2"],"options":{"precision":2}}"#;
    let result = calc.calc_json(input);
    assert_eq!(
      result,
      r#"{"result":[{"value":"3","error":null,"text":[{"string":"1","stype":"Number"},{"string":"+","stype":"Op"},{"string":"2","stype":"Number"}]}]}"#
    );
  }
}
