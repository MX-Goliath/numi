use crate::{calc_units::UnitInfo, localizer::Localizer, Calc};
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

pub struct SiPrefix {
  name: String,
  symbol: String,
  factor: i64,
}

impl SiPrefix {
  fn new(name: &str, symbol: &str, factor: i64) -> Self {
    Self {
      name: name.to_string(),
      symbol: symbol.to_string(),
      factor,
    }
  }

  fn all() -> Vec<SiPrefix> {
    vec![
      SiPrefix::new("yocto", "y", -24),
      SiPrefix::new("zepto", "z", -21),
      SiPrefix::new("atto", "a", -18),
      SiPrefix::new("femto", "f", -15),
      SiPrefix::new("pico", "p", -12),
      SiPrefix::new("nano", "n", -9),
      SiPrefix::new("micro", "μ", -6),
      SiPrefix::new("milli", "m", -3),
      SiPrefix::new("centi", "c", -2),
      SiPrefix::new("deci", "d", -1),
      SiPrefix::new("deca", "da", 1),
      SiPrefix::new("hecto", "h", 2),
      SiPrefix::new("kilo", "k", 3),
      SiPrefix::new("mega", "M", 6),
      SiPrefix::new("giga", "G", 9),
      SiPrefix::new("tera", "T", 12),
      SiPrefix::new("peta", "p", 15),
      SiPrefix::new("exa", "E", 18),
      SiPrefix::new("zetta", "Z", 21),
      SiPrefix::new("yotta", "Y", 24),
    ]
  }

  fn iec_all() -> Vec<SiPrefix> {
    vec![
      SiPrefix::new("kibi", "Ki", 1),
      SiPrefix::new("mebi", "Mi", 2),
      SiPrefix::new("gibi", "Gi", 3),
      SiPrefix::new("tebi", "Ti", 4),
      SiPrefix::new("pebi", "Pi", 5),
      SiPrefix::new("exbi", "Ei", 6),
      SiPrefix::new("zebi", "Zi", 7),
      SiPrefix::new("yobi", "Yi", 8),
    ]
  }
}

impl Calc {
  pub fn si_units_for(unit_info: &UnitInfo, localizer: &Localizer) -> Vec<UnitInfo> {
    let si_prefixes = SiPrefix::all();
    let result = si_prefixes
      .iter()
      .map(|p| Self::si_unit(unit_info, p, localizer))
      .collect();
    result
  }

  pub fn si_unit(unit_info: &UnitInfo, si_prefix: &SiPrefix, localizer: &Localizer) -> UnitInfo {
    let new_id = format!("{}{}", si_prefix.symbol, unit_info.id);

    let symbol_string_id = format!("{}.symbol", si_prefix.name);
    let prefix_string_id = format!("{}.prefix", si_prefix.name);

    let mut loc_format: Option<String> = None;
    let mut loc_phrases: Vec<String> = vec![];
    if let Some(unit_loc_format) = &unit_info.loc_format {
      if let (Some(si_symbol_loc), Some(si_prefix_loc)) = (
        localizer.get_loc_string(&symbol_string_id),
        localizer.get_loc_string(&prefix_string_id),
      ) {
        let format = format!("{}{}", &si_symbol_loc, unit_loc_format);
        loc_format = Some(format.clone());
        loc_phrases = unit_info
          .loc_phrases
          .iter()
          .map(|p| format!("{}{}", &si_prefix_loc, &p))
          .collect();
        loc_phrases.insert(0, format);
      }
    }

    let si_symbol_en = localizer.get_en_string(&symbol_string_id).unwrap();
    let si_prefix_en = localizer.get_en_string(&prefix_string_id).unwrap();
    let en_format = format!("{}{}", &si_symbol_en, unit_info.en_format);

    let mut en_phrases: Vec<String> = vec![];
    for en_phrase in unit_info.en_phrases.iter() {
      let res_phrase = format!("{}{}", &si_prefix_en, &en_phrase);
      en_phrases.push(res_phrase);
    }
    en_phrases.insert(0, en_format.clone());

    let pow_ratio = dec!(10).powi(si_prefix.factor);
    // TODO: check all units that are not supported
    if let Some(new_ratio) = pow_ratio.checked_mul(unit_info.ratio) {
      return UnitInfo {
        id: new_id,
        base_unit_id: unit_info.base_unit_id.clone(),
        ratio: new_ratio,
        loc_format,
        loc_phrases,
        en_format,
        en_phrases,
      };
    }
    UnitInfo {
      id: new_id.clone(),
      base_unit_id: new_id,
      ratio: dec!(1),
      loc_format,
      loc_phrases,
      en_format,
      en_phrases,
    }
  }

  pub fn powered_unit(
    unit_info: &UnitInfo,
    power: i64,
    loc_phrases_template: &[&str],
    loc_format_template: Option<&str>,
    en_phrases_template: &[&str],
    en_format_template: &str,
  ) -> UnitInfo {
    let new_id = format!("{}{}", unit_info.id, power);
    let mut new_base_unit_id = format!("{}{}", unit_info.base_unit_id, power);

    let mut loc_phrases: Vec<String> = vec![];
    for loc_phrase_template in loc_phrases_template.iter() {
      if loc_phrase_template.contains("%@") {
        for phrase in unit_info.loc_phrases.iter() {
          let result_phrase = loc_phrase_template.replace("%@", phrase);
          loc_phrases.push(result_phrase);
        }
      }
    }
    let mut loc_format: Option<String> = None;
    if let (Some(some_loc_format_template), Some(unit_loc_format)) =
      (loc_format_template, &unit_info.loc_format)
    {
      loc_format = Some(some_loc_format_template.replace("%@", unit_loc_format));
    }

    let mut en_phrases: Vec<String> = vec![];
    for phrase_template in en_phrases_template.iter() {
      if phrase_template.contains("%@") {
        for phrase in unit_info.en_phrases.iter() {
          let result_phrase = phrase_template.replace("%@", phrase);
          en_phrases.push(result_phrase);
        }
      }
    }
    let en_format = en_format_template.replace("%@", &unit_info.en_format);

    // Protect from overflow, if happened
    // Disable conversion to base unit
    let mut result_ratio = dec!(1);
    if let Some(ratio) = unit_info.ratio.checked_powi(power) {
      result_ratio = ratio;
    } else {
      new_base_unit_id.clone_from(&new_id);
    }

    UnitInfo {
      id: new_id,
      base_unit_id: new_base_unit_id,
      ratio: result_ratio,
      loc_format,
      loc_phrases,
      en_format,
      en_phrases,
    }
  }

  pub fn iec_units_for(unit_info: &UnitInfo, localizer: &Localizer) -> Vec<UnitInfo> {
    let iec_prefixes = SiPrefix::iec_all();
    let result = iec_prefixes
      .iter()
      .map(|p| Self::iec_unit(unit_info, p, localizer))
      .collect();
    result
  }

  pub fn iec_unit(unit_info: &UnitInfo, iec_prefix: &SiPrefix, localizer: &Localizer) -> UnitInfo {
    let new_id = format!("{}{}", iec_prefix.symbol, unit_info.id);

    let symbol_string_id = format!("{}.symbol", iec_prefix.name);
    let prefix_string_id = format!("{}.prefix", iec_prefix.name);

    let mut loc_format: Option<String> = None;
    let mut loc_phrases: Vec<String> = vec![];
    if let Some(unit_loc_format) = &unit_info.loc_format {
      let si_symbol_loc = localizer.get_loc_string(&symbol_string_id).unwrap();
      let si_prefix_loc = localizer.get_loc_string(&prefix_string_id).unwrap();
      let format = format!("{}{}", &si_symbol_loc, unit_loc_format);
      loc_format = Some(format.clone());
      loc_phrases = unit_info
        .loc_phrases
        .iter()
        .map(|p| format!("{}{}", &si_prefix_loc, &p))
        .collect();
      loc_phrases.insert(0, format);
    }

    let si_symbol_en = localizer.get_en_string(&symbol_string_id).unwrap();
    let si_prefix_en = localizer.get_en_string(&prefix_string_id).unwrap();
    let en_format = format!("{}{}", &si_symbol_en, unit_info.en_format);

    let mut en_phrases: Vec<String> = vec![];
    for en_phrase in unit_info.en_phrases.iter() {
      let res_phrase = format!("{}{}", &si_prefix_en, &en_phrase);
      en_phrases.push(res_phrase);
    }
    en_phrases.insert(0, en_format.clone());

    let new_ratio = unit_info.ratio * dec!(1024).powi(iec_prefix.factor);
    UnitInfo {
      id: new_id,
      base_unit_id: unit_info.base_unit_id.clone(),
      ratio: new_ratio,
      loc_format,
      loc_phrases,
      en_format,
      en_phrases,
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_si_unit() {
    let mut loc = Localizer::new();
    loc.add_loc_string("kilo.symbol", "к");
    loc.add_loc_string("kilo.prefix", "кило");
    loc.add_en_string("kilo.symbol", "k");
    loc.add_en_string("kilo.prefix", "kilo");

    let base_unit_info = UnitInfo {
      id: "m".to_string(),
      base_unit_id: "m".to_string(),
      ratio: dec!(1),
      loc_format: Some("м".to_string()),
      loc_phrases: vec!["метр".to_string(), "метров".to_string()],
      en_format: "m".to_string(),
      en_phrases: vec!["meters".to_string(), "meter".to_string()],
    };
    let si_prefix = SiPrefix::new("kilo", "k", 3);
    let si_unit = Calc::si_unit(&base_unit_info, &si_prefix, &loc);
    let expected_unit_info = UnitInfo {
      id: "km".to_string(),
      base_unit_id: "m".to_string(),
      ratio: dec!(1000),
      loc_format: Some("км".to_string()),
      loc_phrases: vec![
        "км".to_string(),
        "километр".to_string(),
        "километров".to_string(),
      ],
      en_format: "km".to_string(),
      en_phrases: vec![
        "km".to_string(),
        "kilometers".to_string(),
        "kilometer".to_string(),
      ],
    };
    assert_eq!(si_unit, expected_unit_info);
  }
}
