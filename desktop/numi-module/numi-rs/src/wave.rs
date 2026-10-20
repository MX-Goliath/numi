use crate::eval_context::DataContext;
use crate::unit_converter::ValueConverter;
use crate::Value;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const FREQUENCY: &str = "Hz";
const PERIOD: &str = "wave_period";
const WAVELENGTH: &str = "wavelength";
const ANGULAR_FREQUENCY: &str = "angular_frequency";
const WAVE_NUMBER: &str = "wave_number";

/// Converts equivalent descriptions of a wave. Wavelength/frequency
/// conversions use the speed of light in vacuum when no propagation speed is
/// supplied.
#[derive(Clone, Default)]
pub struct WaveConverter;

impl WaveConverter {
  const SPEED_OF_LIGHT: Decimal = dec!(299792458);

  fn result(value: &Value, number: Decimal, unit: &str) -> Value {
    value.clone().with_number(number).with_unit(unit)
  }

  fn si_factor(prefix: &str) -> Option<Decimal> {
    match prefix {
      "y" => Some(dec!(0.000000000000000000000001)),
      "z" => Some(dec!(0.000000000000000000001)),
      "a" => Some(dec!(0.000000000000000001)),
      "f" => Some(dec!(0.000000000000001)),
      "p" => Some(dec!(0.000000000001)),
      "n" => Some(dec!(0.000000001)),
      "μ" | "u" => Some(dec!(0.000001)),
      "m" => Some(dec!(0.001)),
      "c" => Some(dec!(0.01)),
      "d" => Some(dec!(0.1)),
      "da" => Some(dec!(10)),
      "h" => Some(dec!(100)),
      "k" => Some(dec!(1000)),
      "M" => Some(dec!(1000000)),
      "G" => Some(dec!(1000000000)),
      "T" => Some(dec!(1000000000000)),
      _ => None,
    }
  }

  fn frequency_factor(unit: &str) -> Option<Decimal> {
    if unit == FREQUENCY {
      return Some(Decimal::ONE);
    }
    unit.strip_suffix(FREQUENCY).and_then(Self::si_factor)
  }

  fn time_factor(unit: &str) -> Option<Decimal> {
    match unit {
      "sec" => Some(dec!(1)),
      "min" => Some(dec!(60)),
      "hr" => Some(dec!(3600)),
      "day" => Some(dec!(86400)),
      "week" => Some(dec!(604800)),
      "month" => Some(dec!(18144000)),
      "yr" => Some(dec!(220752000)),
      _ => unit.strip_suffix("sec").and_then(Self::si_factor),
    }
  }

  fn length_factor(unit: &str) -> Option<Decimal> {
    match unit {
      "m" => Some(Decimal::ONE),
      "inch" => Some(dec!(0.0254)),
      "ft" => Some(dec!(0.3048)),
      "yd" => Some(dec!(0.9144)),
      "mile" => Some(dec!(1609.344)),
      "point" => Some(dec!(0.00035277777)),
      "cable" => Some(dec!(185.3184)),
      "league" => Some(dec!(4828.032)),
      "furlong" => Some(dec!(201.168)),
      "nmile" => Some(dec!(1853.184)),
      "chain" => Some(dec!(20.1168)),
      _ => unit.strip_suffix('m').and_then(Self::si_factor),
    }
  }

  fn is_time(unit: &str) -> bool {
    Self::time_factor(unit).is_some()
  }

  fn is_length(unit: &str) -> bool {
    Self::length_factor(unit).is_some()
  }

  fn is_wavelength(unit: &str) -> bool {
    unit == WAVELENGTH || Self::is_length(unit)
  }

  fn wavelength_in_m(number: Decimal, unit: &str) -> Option<Decimal> {
    if unit == WAVELENGTH {
      Some(number)
    } else {
      number.checked_mul(Self::length_factor(unit)?)
    }
  }

  fn as_frequency(number: Decimal, unit: &str) -> Option<Decimal> {
    if let Some(factor) = Self::frequency_factor(unit) {
      number.checked_mul(factor)
    } else if unit == PERIOD {
      Decimal::ONE.checked_div(number)
    } else if let Some(factor) = Self::time_factor(unit) {
      Decimal::ONE.checked_div(number.checked_mul(factor)?)
    } else if unit == ANGULAR_FREQUENCY {
      number.checked_div(Decimal::TWO_PI)
    } else if unit == WAVE_NUMBER {
      number
        .checked_mul(Self::SPEED_OF_LIGHT)?
        .checked_div(Decimal::TWO_PI)
    } else if Self::is_wavelength(unit) {
      Self::SPEED_OF_LIGHT.checked_div(Self::wavelength_in_m(number, unit)?)
    } else {
      None
    }
  }
}

impl ValueConverter for WaveConverter {
  fn convert(&self, value: &Value, to: &Value, _context: &DataContext) -> Option<Value> {
    let number = value.number()?;
    let from = value.unit()?;
    let target = to.unit()?;
    if from == target {
      return None;
    }

    // Leave ordinary length and time conversions to the normal ratio converter.
    if Self::is_length(from) && Self::is_length(target)
      || Self::is_time(from) && Self::is_time(target)
    {
      return None;
    }

    let frequency = Self::as_frequency(number, from)?;
    if let Some(target_factor) = Self::frequency_factor(target) {
      return Some(Self::result(
        value,
        frequency.checked_div(target_factor)?,
        target,
      ));
    }
    if target == PERIOD {
      return Some(Self::result(
        value,
        Decimal::ONE.checked_div(frequency)?,
        target,
      ));
    }
    if let Some(target_factor) = Self::time_factor(target) {
      return Some(Self::result(
        value,
        Decimal::ONE
          .checked_div(frequency)?
          .checked_div(target_factor)?,
        target,
      ));
    }
    if target == ANGULAR_FREQUENCY {
      return Some(Self::result(value, frequency * Decimal::TWO_PI, target));
    }
    if target == WAVE_NUMBER {
      return Some(Self::result(
        value,
        frequency
          .checked_mul(Decimal::TWO_PI)?
          .checked_div(Self::SPEED_OF_LIGHT)?,
        target,
      ));
    }
    if Self::is_wavelength(target) {
      let wavelength = Self::SPEED_OF_LIGHT.checked_div(frequency)?;
      let result = if target == WAVELENGTH {
        wavelength
      } else {
        wavelength.checked_div(Self::length_factor(target)?)?
      };
      return Some(Self::result(value, result, target));
    }
    None
  }
}
