use crate::argparser;

const PROGRAM_NAME: &'static str = "{program_name}";
const VERSION: &'static str = "{version}";
const VERSION_MESSAGE: &'static str = "
Unit Converter version {version}
";
const HELP_MESSAGE: &'static str = "
{program_name} [-h|--help] to see how to use.
";
const USAGE_MESSAGE: &'static str = "
[Usage]
{program_name} [input unit] [output unit] [value]
{program_name} [value] [input unit] [output unit]

Example:
  {program_name} m yd 100   convert 100 meters to yards.
  {program_name} 100 m yd   same as the above.

[Unit]
Length
	미터 mm
	야드 yd
	리 里
	해리 海里
Area
	제곱미터 m^2 m2 m²
	헥타르 ha
	에이커 ac
	평
Volume
	리터 L
	갤런 gal
	온스 oz
	되
";

pub fn version_message() -> String {
    VERSION_MESSAGE.replace(VERSION, env!("CARGO_PKG_VERSION"))
}

pub fn usage_message() -> String {
    USAGE_MESSAGE.replace(PROGRAM_NAME, argparser::program_name().as_str())
}

pub fn help_message() -> String {
    HELP_MESSAGE.replace(PROGRAM_NAME, argparser::program_name().as_str())
}
