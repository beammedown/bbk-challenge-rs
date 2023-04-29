#![feature(int_roundings)]

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::algorithm::Algorithm;
use crate::parser::parse;

pub mod algorithm;
pub mod output;
pub mod parser;

fn run(name: &str) -> std::io::Result<()> {
	let output = Algorithm::execute(parse(&fs::read_to_string(
		Path::new("challenges").join(format!("{name}.txt")),
	)?));
	println!("{name}: {}", output.total_value);
	write!(
		File::create(Path::new("out").join(format!("{name}.out.txt")))?,
		"{output}"
	)?;
	Ok(())
}

fn main() {
	let handle_a = std::thread::spawn(|| {
		run("a_example").unwrap();
	});
	let handle_b = std::thread::spawn(|| {
		run("b_read_on").unwrap();
	});
	let handle_c = std::thread::spawn(|| {
		run("c_incunabula").unwrap();
	});
	let handle_d = std::thread::spawn(|| {
		run("d_tough_choices").unwrap();
	});
	let handle_e = std::thread::spawn(|| {
		run("e_so_many_coins").unwrap();
	});
	let handle_f = std::thread::spawn(|| {
		run("f_banks_of_the_world").unwrap();
	});
	for handle in [handle_a, handle_b, handle_c, handle_d, handle_e, handle_f] {
		handle.join().unwrap();
	}
}
