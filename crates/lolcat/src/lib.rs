use std::f64::consts;
use std::io::{stdout, Write};
use std::process::exit;

use colored::Color;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

fn rainbow(freq: f64, i: f64) -> (u8, u8, u8) {
	let red = ((freq * i).sin() * 127.0) as i16 + 128;
	let green = (freq.mul_add(i, 2.0 * consts::PI / 3.0).sin() * 127.0) as i16 + 128;
	let blue = (freq.mul_add(i, 4.0 * consts::PI / 3.0).sin() * 127.0) as i16 + 128;
	(red as u8, green as u8, blue as u8)
}

pub fn print_rainbow(line: &str, freq: f64, seed: f64, spread: f64, invert: bool) {
	// regex taken from ruby version - https://github.com/busyloop/lolcat/blob/master/lib/lolcat/lol.rb#L30
	lazy_static! {
		static ref ANSI_ESCAPE: Regex = RegexBuilder::new("((?:\x1B(?:[ -/]+.|[]PX^_][^\x07\x1B]*|\\[[0-?]*.|.))*)(.?)")
			.build()
			.unwrap();
	};

	for (i, c) in ANSI_ESCAPE.captures_iter(line).enumerate() {
		let (r, g, b) = rainbow(freq, seed + i as f64 / spread);
		let color = Color::TrueColor { r, g, b };

		let result = if invert {
			stdout().write_all(format!("{}\x1B[{}m{}\x1B[49m", &c[1], color.to_bg_str(), &c[2]).as_bytes())
		} else {
			stdout().write_all(format!("{}\x1B[{}m{}\x1B[39m", &c[1], color.to_fg_str(), &c[2]).as_bytes())
		};
		if result.is_err() {
			exit(exitcode::IOERR);
		}
	}
}
