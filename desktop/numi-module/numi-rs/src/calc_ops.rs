use crate::{localizer::Localizer, op::Op, Calc};

impl Calc {
  pub fn add_ops(&self, localizer: &Localizer) {
    let mut decimal_op = self.decimal_op.borrow_mut();
    decimal_op.add_op(
      Op::new("+"),
      Some(10),
      localizer.get_strings("plus.variants"),
    );
    decimal_op.add_op(
      Op::new("-"),
      Some(10),
      localizer.get_strings("minus.variants"),
    );

    let mut mul_phrases = localizer.get_strings("multiply.variants");
    mul_phrases.push("X");
    mul_phrases.push("x");

    decimal_op.add_op(Op::new("*"), Some(20), mul_phrases);
    decimal_op.add_op(
      Op::new("/"),
      Some(20),
      localizer.get_strings("divide.variants"),
    );
    decimal_op.add_op(Op::new("("), None, vec!["("]);
    decimal_op.add_op(Op::new(")"), None, vec![")"]);
    decimal_op.add_op(
      Op::new("in"),
      None,
      localizer.get_strings("conversion.variants"),
    );
    decimal_op.add_op(
      Op::new("="),
      Some(100),
      localizer.get_strings("assignment.variants"),
    );

    decimal_op.add_op(Op::new("exp"), Some(40), vec!["^", "ˆ"]);
    decimal_op.add_op(Op::new("mod"), Some(30), vec!["mod"]);
    decimal_op.add_op(Op::new("and"), Some(30), vec!["&"]);
    decimal_op.add_op(Op::new("or"), Some(30), vec!["|"]);
    decimal_op.add_op(Op::new("xor"), Some(30), vec!["xor"]);
    decimal_op.add_op(Op::new("lshift"), Some(30), vec!["<<"]);
    decimal_op.add_op(Op::new("rshift"), Some(30), vec![">>"]);

    let percent_weight = Option::Some(1);
    decimal_op.add_op(
      Op::new("of"),
      percent_weight,
      localizer.get_strings("percent_of.variants"),
    );
    decimal_op.add_op(
      Op::new("on"),
      percent_weight,
      localizer.get_strings("percent_on.variants"),
    );
    decimal_op.add_op(
      Op::new("off"),
      percent_weight,
      localizer.get_strings("percent_off.variants"),
    );
    decimal_op.add_op(
      Op::new("aspof"),
      percent_weight,
      localizer.get_strings("as_a_percent_of.variants"),
    );
    decimal_op.add_op(
      Op::new("aspon"),
      percent_weight,
      localizer.get_strings("as_a_percent_on.variants"),
    );
    decimal_op.add_op(
      Op::new("aspoff"),
      percent_weight,
      localizer.get_strings("as_a_percent_off.variants"),
    );
    decimal_op.add_op(
      Op::new("pofw"),
      percent_weight,
      localizer.get_strings("percent_of_what_is.variants"),
    );
    decimal_op.add_op(
      Op::new("ponw"),
      percent_weight,
      localizer.get_strings("percent_on_what_is.variants"),
    );
    decimal_op.add_op(
      Op::new("poffw"),
      percent_weight,
      localizer.get_strings("percent_off_what_is.variants"),
    );
  }
}
