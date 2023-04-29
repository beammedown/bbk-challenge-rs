#![feature(int_roundings)]

//use algorithms::idee1::Idee1;

use crate::algorithms::idee2::Algorithm as Idee2;
use crate::parser::{parse};

pub mod algorithms;
pub mod output;
pub mod parser;

fn main() {
	Idee2::execute(parse(include_str!("../challenges/d_tough_choices.txt")));
}
