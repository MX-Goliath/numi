use crate::{localizer::Localizer, utils::VecStrToString, Calc, Unit};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitInfo {
  pub id: String,
  pub base_unit_id: String,
  pub ratio: Decimal,
  pub loc_format: Option<String>,
  pub loc_phrases: Vec<String>,
  pub en_format: String,
  pub en_phrases: Vec<String>,
}

impl UnitInfo {
  pub fn new(
    id: &str,
    base_unit_id: &str,
    ratio: Decimal,
    format: Option<&str>,
    phrases: Vec<&str>,
    en_format: &str,
    en_phrases: Vec<&str>,
  ) -> Self {
    let mut loc_format: Option<String> = None;
    if let Some(some_format) = format {
      loc_format = Some(some_format.to_string());
    }
    UnitInfo {
      id: id.to_string(),
      loc_format,
      base_unit_id: base_unit_id.to_string(),
      ratio,
      loc_phrases: phrases.to_strings(),
      en_format: en_format.to_string(),
      en_phrases: en_phrases.to_strings(),
    }
  }
}

impl Calc {
  pub fn add_units(&self, localizer: &Localizer) {
    self.add_unit(
      Unit::new("%", "%"),
      &localizer.get_strings("percent.variants"),
    );

    // Length
    let length_units = self.length_units(localizer);
    let squared_units = length_units
      .iter()
      .map(|u| self.square_unit(u, localizer))
      .collect::<Vec<UnitInfo>>();
    let volume_units = length_units
      .iter()
      .map(|u| self.volume_unit(u, localizer))
      .collect::<Vec<UnitInfo>>();
    self.add_unit_infos(&length_units);
    self.add_unit_infos(&squared_units);
    self.add_unit_infos(&volume_units);

    self.add_other_length_units(localizer);
    self.add_area_units(localizer);
    self.add_volume_units(localizer);
    self.add_weight_units(localizer);
    self.add_angular_units(localizer);
    self.add_data_units(localizer);
    self.add_time_units(localizer);
    self.add_wave_units();
  }

  fn length_units(&self, localizer: &Localizer) -> Vec<UnitInfo> {
    let mut res = vec![];

    let meter_info = self.build_unit_info("m", "m", dec!(1), "meter");
    res.append(&mut self.si_units(&meter_info, localizer));
    res.push(meter_info);

    res.push(self.build_unit_info("inch", "m", dec!(0.0254), "inch"));
    res.push(self.build_unit_info("ft", "m", dec!(0.3048), "foot"));
    res.push(self.build_unit_info("yd", "m", dec!(0.9144), "yard"));
    res.push(self.build_unit_info("mile", "m", dec!(1609.344), "mile"));
    res
  }

  fn add_other_length_units(&self, _localizer: &Localizer) {
    self.add_unit_info(&self.build_unit_info("point", "m", dec!(0.00035277777), "point"));
    self.add_unit_info(&self.build_unit_info("cable", "m", dec!(185.3184), "cable"));
    self.add_unit_info(&self.build_unit_info("league", "m", dec!(4828.032), "league"));
    self.add_unit_info(&self.build_unit_info("furlong", "m", dec!(201.168), "furlong"));
    self.add_unit_info(&self.build_unit_info("nmile", "m", dec!(1853.184), "nautical_mile"));
    self.add_unit_info(&self.build_unit_info("chain", "m", dec!(20.1168), "chain"));
  }

  fn add_area_units(&self, _localizer: &Localizer) {
    self.add_unit_info(&self.build_unit_info("acre", "m2", dec!(4046.86), "acre"));
    self.add_unit_info(&self.build_unit_info("hectare", "m2", dec!(10000), "hectare"));
    self.add_unit_info(&self.build_unit_info("are", "m2", dec!(100), "are"));
    self.add_unit_info(&self.build_unit_info("ping", "m2", dec!(3.306), "ping"));
  }

  fn add_volume_units(&self, localizer: &Localizer) {
    let litre = self.build_unit_info("litre", "m3", dec!(0.001), "litre");
    self.add_unit_infos(&self.si_units(&litre, localizer));
    self.add_unit_info(&litre);

    self.add_unit_info(&self.build_unit_info("cup", "m3", dec!(0.000236588237), "cup"));
    self.add_unit_info(&self.build_unit_info("quart", "m3", dec!(0.000946352946), "quart"));
    self.add_unit_info(&self.build_unit_info("pint", "m3", dec!(0.000473176473), "pint"));
    self.add_unit_info(&self.build_unit_info("gallon", "m3", dec!(0.00378541178), "gallon"));
    self.add_unit_info(&self.build_unit_info(
      "tablespoon",
      "m3",
      dec!(0.0000147867648),
      "tablespoon",
    ));
    self.add_unit_info(&self.build_unit_info("teaspoon", "m3", dec!(0.00000492892159), "teaspoon"));
    let mut unit_manager = self.unit_manager.borrow_mut();
    unit_manager.add_phrases("inch3", &localizer.get_strings("cbinch.variants"));
    unit_manager.add_phrases("cm3", &localizer.get_strings("cbcm.variants"));
  }

  fn add_weight_units(&self, localizer: &Localizer) {
    let gram = self.build_unit_info("gram", "gram", dec!(1), "gram");
    self.add_unit_infos(&self.si_units(&gram, localizer));
    self.add_unit_info(&gram);

    let tonne = self.build_unit_info("tonne", "gram", dec!(1000000), "tonne");
    // TODO: There are some exclusions for tonne's ft,pt,ct symbols
    self.add_unit_infos(&self.si_units(&tonne, localizer));
    self.add_unit_info(&tonne);

    self.add_unit_info(&self.build_unit_info("carat", "gram", dec!(0.2), "carat"));
    self.add_unit_info(&self.build_unit_info("pound", "gram", dec!(453.59237), "pound"));
    self.add_unit_info(&self.build_unit_info("centner", "gram", dec!(100000), "centner"));
    self.add_unit_info(&self.build_unit_info("ounce", "gram", dec!(28.349523125), "ounce"));
    self.add_unit_info(&self.build_unit_info("stone", "gram", dec!(6350.29318), "stone"));
  }

  fn add_angular_units(&self, _localizer: &Localizer) {
    self.add_unit_info(&self.build_unit_info("radian", "radian", dec!(1), "radian"));
    self.add_unit_info(&self.build_unit_info(
      "degree",
      "radian",
      Decimal::PI / dec!(180),
      "degree",
    ));
  }

  fn add_data_units(&self, localizer: &Localizer) {
    let bit = self.build_unit_info("bit", "bit", dec!(1), "bit");
    self.add_unit_infos(&self.si_units(&bit, localizer));
    self.add_unit_infos(&Calc::iec_units_for(&bit, localizer));
    self.add_unit_info(&bit);

    let byte = self.build_unit_info("byte", "bit", dec!(8), "byte");
    self.add_unit_infos(&self.si_units(&byte, localizer));
    self.add_unit_infos(&Calc::iec_units_for(&byte, localizer));
    self.add_unit_info(&byte);
  }

  fn add_time_units(&self, localizer: &Localizer) {
    let second = self.build_unit_info("sec", "sec", dec!(1), "second");
    self.add_unit_infos(&self.si_units(&second, localizer));
    self.add_unit_info(&second);

    self.add_unit_info(&self.build_unit_info("min", "sec", dec!(60), "minute"));

    self.add_unit_info(&self.build_unit_info("hr", "sec", dec!(3600), "hour"));
    self.add_unit_info(&self.build_unit_info("day", "sec", dec!(86400), "day"));
    self.add_unit_info(&self.build_unit_info("week", "sec", dec!(604800), "week"));
    self.add_unit_info(&self.build_unit_info("month", "sec", dec!(18144000), "month"));
    self.add_unit_info(&self.build_unit_info("yr", "sec", dec!(220752000), "year"));
  }

  fn add_wave_units(&self) {
    self.add_unit(
      Unit::new("Hz", "Hz"),
      &vec![
        "Hz",
        "hz",
        "hertz",
        "frequency",
        "frequencies",
        "cycles per second",
        "cps",
      ],
    );
    for (id, name) in [
      ("mHz", "millihertz"),
      ("μHz", "microhertz"),
      ("nHz", "nanohertz"),
      ("kHz", "kilohertz"),
      ("MHz", "megahertz"),
      ("GHz", "gigahertz"),
      ("THz", "terahertz"),
    ] {
      let lower = id.to_lowercase();
      self.add_unit(Unit::new(id, "Hz"), &vec![id, lower.as_str(), name]);
    }
    self.add_unit(
      Unit::new("wave_period", "s"),
      &vec![
        "period",
        "periods",
        "wave period",
        "wave periods",
        "T",
        "seconds per cycle",
      ],
    );
    self.add_unit(
      Unit::new("wavelength", "m"),
      &vec!["wavelength", "wavelengths", "lambda", "λ"],
    );
    self.add_unit(
      Unit::new("angular_frequency", "rad/s"),
      &vec!["angular frequency", "angular frequencies", "omega", "ω"],
    );
    self.add_unit(
      Unit::new("wave_number", "rad/m"),
      &vec![
        "wave number",
        "wave numbers",
        "wavenumber",
        "wavenumbers",
        "k",
      ],
    );
  }

  fn si_units(&self, unit_info: &UnitInfo, localizer: &Localizer) -> Vec<UnitInfo> {
    Calc::si_units_for(unit_info, localizer)
  }

  fn square_unit(&self, unit_info: &UnitInfo, localizer: &Localizer) -> UnitInfo {
    Calc::powered_unit(
      unit_info,
      2,
      &localizer.get_loc_strings("area.templates"),
      localizer.get_loc_string("area.format"),
      &localizer.get_en_strings("area.templates"),
      localizer.get_en_string("area.format").unwrap(),
    )
  }

  fn volume_unit(&self, unit_info: &UnitInfo, localizer: &Localizer) -> UnitInfo {
    Calc::powered_unit(
      unit_info,
      3,
      &localizer.get_loc_strings("volume.templates"),
      localizer.get_loc_string("volume.format"),
      &localizer.get_en_strings("volume.templates"),
      localizer.get_en_string("volume.format").unwrap(),
    )
  }

  fn build_unit_info(
    &self,
    id: &str,
    base_unit_id: &str,
    ratio: Decimal,
    string_id: &str,
  ) -> UnitInfo {
    let format_string_id = format!("{}.format", string_id);
    let phrases_string_id = format!("{}.variants", string_id);
    UnitInfo::new(
      id,
      base_unit_id,
      ratio,
      self.localizer.get_loc_string(&format_string_id),
      self.localizer.get_loc_strings(&phrases_string_id),
      self.localizer.get_en_string(&format_string_id).unwrap(),
      self.localizer.get_en_strings(&phrases_string_id),
    )
  }

  fn add_unit_infos(&self, infos: &[UnitInfo]) {
    for info in infos.iter() {
      self.add_unit_info(info);
    }
  }

  fn add_unit_info(&self, unit_info: &UnitInfo) {
    let mut ratio_converter = self.ratio_converter.borrow_mut();
    ratio_converter.set_ratio(&unit_info.id, &unit_info.base_unit_id, unit_info.ratio);
    let mut unit_manager = self.unit_manager.borrow_mut();

    let mut format = unit_info.en_format.clone();
    if let Some(loc_format) = &unit_info.loc_format {
      format.clone_from(loc_format);
    }
    let unit = Unit::new(&unit_info.id, &format);

    let mut phrases = vec![];
    for p in &unit_info.loc_phrases {
      phrases.push(p.as_str());
    }
    for p in &unit_info.en_phrases {
      phrases.push(p.as_str());
    }
    unit_manager.add_unit(unit, &phrases);
  }
}
