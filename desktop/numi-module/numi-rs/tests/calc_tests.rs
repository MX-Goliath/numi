#![allow(dead_code)]

use numi::*;
use regex::Regex;

/*
 * Initialize env logger to show debug! output:
 *
 * ```
 * let _ = env_logger::builder().is_test(true).try_init();
 * ```
*/

#[test]
fn test_number() {
  assert_eq!(calculate_format("5"), "5");
  assert_eq!(calculate_format(" 5 "), "5");
  assert_eq!(calculate_format("-5"), "-5");
  assert_eq!(calculate_format("+5"), "5");
  assert_eq!(calculate_format("1 000"), "1000");
  assert_eq!(calculate_format("1,000"), "1000");
}

#[test]
fn test_basic_arithmetics_scenarios() {
  assert_eq!(calculate_format("2*-3"), "-6");
  assert_eq!(calculate_format("2+3"), "5");
  assert_eq!(calculate_format("5-2"), "3");
  assert_eq!(calculate_format("2*3"), "6");
  assert_eq!(calculate_format("6/3"), "2");
  assert_eq!(calculate_format("1 + 2"), "3");
  assert_eq!(calculate_format("1 + 2 + 3"), "6");
  assert_eq!(calculate_format("1.1 + 2.2"), "3.3");
  assert_eq!(calculate_format("2 and 3"), "5");
  assert_eq!(calculate_format("2 times 3"), "6");
  assert_eq!(calculate_format("2x3"), "6");
  assert_eq!(calculate_format("2X3"), "6");
}

#[test]
fn test_decimal_format() {
  assert_eq!(calculate_format("1+2"), "3");
  assert_eq!(calculate_format("1%"), "1 %");
}

#[test]
fn test_op_order() {
  assert_eq!(calculate_format("1+2*3"), "7");
  assert_eq!(calculate_format("9-2*3"), "3");
  assert_eq!(calculate_format("1+2*3-4"), "3");
  assert_eq!(calculate_format("20-5+1"), "16");
  assert_eq!(calculate_format("4*3/2"), "6");
  assert_eq!(calculate_format("(2+3)*4-5+1"), "16");
  assert_eq!(calculate_format("4*3-2/2"), "11");
}

#[test]
fn test_decimal_ops() {
  assert_eq!(calculate_format("1 m + 20 cm"), "120 cm");
  assert_eq!(calculate_format("1 m - 20 cm"), "80 cm");
  assert_eq!(calculate_format("3^2"), "9");
  assert_eq!(calculate_format("5 mod 2"), "1");
}

#[test]
fn test_bitwise_ops() {
  assert_eq!(calculate_format("3&2"), "2");
  assert_eq!(calculate_format("1|2"), "3");
  assert_eq!(calculate_format("2 xor 3"), "1");
  assert_eq!(calculate_format("2 >> 1"), "1");
  assert_eq!(calculate_format("2 << 1"), "4");
}

#[test]
fn test_conversion_op_order() {
  assert_eq!(calculate_format("1 m in cm + 20"), "120 cm");
  assert_eq!(calculate_format("20 + 1 m in cm"), "120 cm");
}

#[test]
fn test_parentheses() {
  assert_eq!(calculate_format("(2)"), "2");
  assert_eq!(calculate_format("(2"), "");
  assert_eq!(calculate_format("((2))"), "2");
  assert_eq!(calculate_format("(-2+4)*3"), "6");
  assert_eq!(calculate_format("-(1+2)"), "-3");
  assert_eq!(calculate_format("2*(1+2)"), "6");
  assert_eq!(calculate_format("(1+2)*2"), "6");
  assert_eq!(calculate_format("(1+2)*(1+2)"), "9");
  assert_eq!(calculate_format("(10-10%)"), "9");
}

#[test]
fn test_units() {
  assert_eq!(calculate_format("1 meter"), "1 m");
  assert_eq!(calculate_format("1m"), "1 m");
  assert_eq!(calculate_format("1 in m"), "1 m");
  assert_eq!(calculate_format("2 dm in cm"), "20 cm");
  assert_eq!(calculate_format("2 cm in kg"), "");
  assert_eq!(calculate_format("-1m"), "-1 m");
  assert_eq!(calculate_format("m 1"), "1 m");
}

#[test]
fn test_percentage() {
  assert_eq!(calculate_format("2+50%"), "3");
  assert_eq!(calculate_format("2m+50%"), "3 m");
  assert_eq!(calculate_format("50% of 2 cm"), "1.00 cm");
  assert_eq!(calculate_format("50% on 2"), "3.00");
  assert_eq!(calculate_format("25% off 4"), "3.00");
  assert_eq!(calculate_format("5 as a % of 50"), "10.00 %");
  assert_eq!(calculate_format("20 cm as a % of 2 m"), "10.00 %");
  assert_eq!(calculate_format("3 as a % on 2"), "50.00 %");
  assert_eq!(calculate_format("1 as a % off 2"), "50.00 %");
  assert_eq!(calculate_format("50% of what is 1"), "2");
  assert_eq!(calculate_format("50% on what is 3m"), "2 m");
  assert_eq!(calculate_format("50% off what is 1"), "2");
}

#[test]
fn test_unit_detection() {
  assert_eq!(calculate_format("2 percents"), "2 %");
}

#[test]
fn test_word_operations() {
  assert_eq!(calculate_format("1 plus 2"), "3");
}

#[test]
fn test_unit_conversion() {
  assert_eq!(calculate_format("2 m in cm"), "200 cm");
  assert_eq!(calculate_format("10 celsius in fahrenheit"), "50.00 °F");
  assert_eq!(calculate_format("5 m 5 cm"), "505 cm");
  assert_eq!(calculate_format("2m 2cm"), "202 cm");
  assert_eq!(calculate_format("2m in celsius"), "");
}

#[test]
fn test_length_units() {
  assert_eq!(calculate_format("1 mile in km"), "1.61 km");
  assert_eq!(calculate_format("2 feet in m"), "0.61 m");
  assert_eq!(calculate_format("10000 inch in m"), "254.00 m");
  assert_eq!(calculate_format("10000 foots in m"), "3048.00 m");
  assert_eq!(calculate_format("1000 miles in m"), "1609344.00 m");
}

#[test]
fn test_functions() {
  assert_eq!(calculate_format("sqrt 9"), "3.00");
  assert_eq!(calculate_format("cbrt 8"), "2");
  assert_eq!(calculate_format("round 1.3"), "1");
  assert_eq!(calculate_format("abs -1"), "1");
  assert_eq!(calculate_format("ln 10"), "2.30");
  assert_eq!(calculate_format("ceil 1.1"), "2");
  assert_eq!(calculate_format("floor 1.9"), "1");
  // TODO: add support for multi-arg parsing
  // assert_eq!(calculate_format("log 2 (8)"), "3");

  // TODO: Add localization for common functions
  // assert_eq!(calculate_format("square root 9"), "3.00");
}

#[test]
fn test_dates() {
  assert_eq!(calculate_format("2023-04-05"), "2023-04-05");
  assert_eq!(calculate_format("2023-04-05 + 1 day"), "2023-04-06");
  assert_eq!(calculate_format("2023-04-05 - 1 day"), "2023-04-04");
  assert!(calculate_format("now").contains("-"));
  assert!(calculate_format("time").contains("-"));

  let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
  assert!(date_regex.is_match(calculate_format("today").as_str()));
  assert!(date_regex.is_match(calculate_format("tomorrow").as_str()));
  assert!(date_regex.is_match(calculate_format("yesterday").as_str()));
  assert_ne!(calculate_format("today"), calculate_format("tomorrow"));
  assert_ne!(calculate_format("today"), calculate_format("yesterday"));
  assert_ne!(calculate_format("tomorrow"), calculate_format("yesterday"));
  assert_eq!(
    calculate_format("today + 1 day"),
    calculate_format("tomorrow")
  );
  assert_eq!(
    calculate_format("today - 1 day"),
    calculate_format("yesterday")
  );
}

#[test]
fn test_datetime() {
  assert_eq!(
    calculate_format("2023-04-05 10:20:30"),
    "2023-04-05 10:20:30"
  );
}

#[test]
fn test_calc_timezone() {
  let mut calc = Calc::new();
  let current_time = "2023-01-02 11:12:13";
  calc
    .data_context
    .set_default_timezone("America/Los_Angeles");
  calc
    .data_context
    .set_current_datetime(DateTime::with_str_datetime(current_time).unwrap());
  assert_eq!(
    calc
      .calculate_format("2023-04-05 10:20:30 in UTC", &FormatOptions::default())
      .unwrap_or_default(),
    "2023-04-05 17:20:30"
  );
  assert_eq!(
    calc
      .calculate_format(
        "2023-04-05 10:20:30 in Asia/Tokyo",
        &FormatOptions::default()
      )
      .unwrap(),
    "2023-04-06 02:20:30"
  );
  assert_eq!(
    calc
      .calculate_format("UTC time", &FormatOptions::default())
      .unwrap(),
    "2023-01-02 19:12:13"
  );
  assert_eq!(
    calc
      .calculate_format("time in UTC", &FormatOptions::default())
      .unwrap(),
    "2023-01-02 19:12:13"
  );
  assert!(calc
    .calculate_format("10:30 in UTC", &FormatOptions::default())
    .unwrap()
    .contains(":"));
}

#[test]
fn test_trig_fns() {
  assert_eq!(calculate_format("sin (PI/6)"), "0.5");
}

#[test]
fn test_negative_numbers() {
  assert_eq!(calculate_format("-1"), "-1");
  assert_eq!(calculate_format("-1 * 3"), "-3");
}

#[test]
fn test_si_units() {
  assert_eq!(calculate_format("1 km in m"), "1000 m");
  assert_eq!(calculate_format("2 meters in centimeters"), "200 cm");
}

#[test]
fn test_wave_units() {
  assert_eq!(calculate_format("2 Hz in period"), "0.50 s");
  assert_eq!(calculate_format("1 Hz in wavelength"), "299792458 m");
  assert_eq!(calculate_format("1 wavelength in Hz"), "299792458 Hz");
  assert_eq!(calculate_format("1 Hz in angular frequency"), "6.28 rad/s");
  assert_eq!(
    calculate_format("1 wavelength in wave number"),
    "6.28 rad/m"
  );
  assert_eq!(calculate_format("1 angular frequency in Hz"), "0.16 Hz");
  assert_eq!(calculate_format("1 wave number in wavelength"), "6.28 m");
  assert_eq!(calculate_format("1 Hz in ms"), "1000 ms");
  assert_eq!(calculate_format("0 Hz in wavelength"), "");
  assert_eq!(calculate_format("1 Hz in m"), "299792458 m");
  assert_eq!(calculate_format("1 Hz in s"), "1 s");
  assert_eq!(calculate_format("1 Hz in km"), "299792.46 km");
  assert_eq!(calculate_format("1 kHz in m"), "299792.46 m");
  assert_eq!(calculate_format("1 kHz in Hz"), "1000 Hz");
}

#[test]
fn test_vars() {
  assert!(calculate_format("pi").contains("3.14"));
  assert!(calculate_format("a = 1").contains("1"));
  assert!(calculate_format("meter = 2").contains("2"));
}

#[test]
fn test_var_assignment() {
  let calc = Calc::new();
  let output = calc.calc(&CalcInput::with_lines(vec!["aaa = 2*3", "aaa"]));
  assert_eq!(output.result.len(), 2);
  assert_eq!(output.result[0].value, Some("6".to_string()));
  assert_eq!(output.result[1].value, Some("6".to_string()));
}

#[test]
fn test_scales() {
  assert_eq!(calculate_format("1k"), "1000");
  assert_eq!(calculate_format("1 million"), "1000000");
  assert_eq!(calculate_format("1k m"), "1000 m");
  assert_eq!(calculate_format("1 M"), "1000000");
}

#[test]
fn test_squared_units() {
  assert_eq!(calculate_format("1 sqm"), "1 m²");
  assert_eq!(calculate_format("1 sqm in sqcm"), "10000 cm²");
  // Phrase detector improve required
  // with split by space only first, and then split by space
  // and numbers + join results
  // assert_eq!(calculate_format("1 m2"), "1 m²");
}

#[test]
fn test_volume_units() {
  assert_eq!(calculate_format("1 cbm"), "1 m³");
  assert_eq!(calculate_format("1 cbm in cbcm"), "1000000 cm³");
  assert_eq!(calculate_format("2 L in mL"), "2000 mL");
  // Phrase detector improve required
  // with split by space only first, and then split by space
  // and numbers + join results
  // assert_eq!(calculate_format("1 m3"), "1 m³");
  // assert_eq!(calculate_format("2 m3 in litres"), "2000 L");
  // assert_eq!(calculate_format("1 m3 in ccm"), "1000000 cm³");
}

#[test]
fn test_weight_units() {
  assert_eq!(calculate_format("1g in mg"), "1000 mg");
  assert_eq!(calculate_format("1 tonne in kg"), "1000 kg");
}

#[test]
fn test_angle_units() {
  assert_eq!(calculate_format("180 degrees in radians"), "3.14 rad");
}

#[test]
fn test_time_units() {
  assert_eq!(calculate_format("2 minutes in ms"), "120000 ms");
  assert_eq!(calculate_format("1 week in hours"), "168 h");
}

#[test]
fn test_data_units() {
  assert_eq!(calculate_format("1 byte in bits"), "8 b");
  assert_eq!(calculate_format("1 kb in bits"), "1000 b");
  assert_eq!(calculate_format("1 Kib in bits"), "1024 b");
  assert_eq!(calculate_format("1 kB in bytes"), "1000 B");
  assert_eq!(calculate_format("1 KiB in bytes"), "1024 B");
}

#[test]
fn test_repr() {
  assert_eq!(calculate_format("2.5e2"), "2.50e2");
  assert_eq!(calculate_format("2.5e2 in decimal"), "250");
  assert_eq!(calculate_format("250 in scientific"), "2.50e2");
  assert_eq!(calculate_format("0b1111 in decimal"), "15");
  assert_eq!(calculate_format("15 in binary"), "0b1111");
  assert_eq!(calculate_format("0o5 in decimal"), "5");
  assert_eq!(calculate_format("5 in octal"), "0o5");
  assert_eq!(calculate_format("0xff in decimal"), "255");
  assert_eq!(calculate_format("255 in hex"), "0xff");
}

#[test]
fn test_currency() {
  let mut calc = Calc::new();
  calc.set_context_value("USD", &Value::default().with_int(1));
  calc.set_context_value("BYN", &Value::default().with_int(2));
  calc.add_unit(Unit::new("USD", "USD"), &vec!["USD"]);
  calc.add_unit(Unit::new("BYN", "BYN"), &vec!["BYN"]);
  assert_eq!(
    calc
      .calculate_format("3 USD in BYN", &FormatOptions::default())
      .unwrap_or_default(),
    "6 BYN"
  );
  assert_eq!(
    calc
      .calculate_format("4 BYN in USD", &FormatOptions::default())
      .unwrap_or_default(),
    "2 USD"
  );
}

#[test]
fn test_currency_from_json() {
  let json_str = r#"{
		"timestamp": 1675098000,
		"base": "USD",
		"rates": {
			"BYN": 2,
			"USD": 1
		}
	}"#;
  let mut calc = Calc::new();
  calc.fill_rates_from_json_str(json_str);
  assert_eq!(
    calc
      .calculate_format("3 USD in BYN", &FormatOptions::default())
      .unwrap_or_default(),
    "6 BYN"
  );
  assert_eq!(
    calc
      .calculate_format("3 usd in byn", &FormatOptions::default())
      .unwrap_or_default(),
    "6 BYN"
  );
  assert_eq!(
    calc
      .calculate_format("3 dollars in byn", &FormatOptions::default())
      .unwrap_or_default(),
    "6 BYN"
  );
  // assert_eq!(calc.calculate_format("3 USD in bel rubles"), "6 BYN");
}

#[test]
fn test_prev_token() {
  let calc = Calc::new();
  let output = calc.calc(&CalcInput::with_lines(vec!["1+2", "prev+2", "prev+1"]));
  assert_eq!(output.result.len(), 3);
  assert_eq!(output.result[0].value, Some("3".to_string()));
  assert_eq!(output.result[1].value, Some("5".to_string()));
  assert_eq!(output.result[2].value, Some("6".to_string()));
}

#[test]
fn test_sum_token() {
  let calc = Calc::new();
  let output = calc.calc(&CalcInput::with_lines(vec!["1+2", "3", "sum + 1"]));
  assert_eq!(output.result.len(), 3);
  assert_eq!(output.result[0].value, Some("3".to_string()));
  assert_eq!(output.result[1].value, Some("3".to_string()));
  assert_eq!(output.result[2].value, Some("7".to_string()));

  let output2 = calc.calc(&CalcInput::with_lines(vec!["1", "", "2", "sum"]));
  assert_eq!(output2.result.len(), 4);
  assert_eq!(output2.result[0].value, Some("1".to_string()));
  assert_eq!(output2.result[1].value, None);
  assert_eq!(output2.result[2].value, Some("2".to_string()));
  assert_eq!(output2.result[3].value, Some("2".to_string()));
}

#[test]
fn test_avg_token() {
  let calc = Calc::new();
  let output = calc.calc(&CalcInput::with_lines(vec!["2", "4", "avg"]));
  assert_eq!(output.result.len(), 3);
  assert_eq!(output.result[0].value, Some("2".to_string()));
  assert_eq!(output.result[1].value, Some("4".to_string()));
  assert_eq!(output.result[2].value, Some("3".to_string()));
}

fn future_test() {
  assert_eq!(calculate_format("2 l in ml"), "2000 mL");
}

fn calculate_format(str: &str) -> String {
  let calc = Calc::new();
  calc
    .calculate_format(str, &FormatOptions::default())
    .unwrap_or_default()
}
