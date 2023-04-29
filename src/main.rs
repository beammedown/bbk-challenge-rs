#![feature(int_roundings)]

use algorithms::idee1::Idee1;

use crate::algorithms::idee2::Idee2;
use crate::algorithms::Algorithm;
use crate::parser::{parse, Bank, Challenge};

pub mod algorithms;
pub mod output;
pub mod parser;

fn main() {
	Idee1::run(parse(include_str!("../challenges/b_read_on.txt")));
	println!(
		"{:#?}",
		Idee2::run(Challenge {
			days_to_scan: 7,
			coins: vec![12, 5, 8, 10, 9, 3, 2, 1, 4, 6],
			banks: vec![
				Bank {
					index: 0,
					scans_per_day: 1,
					registration_time: 2,
					coins: vec![0, 2, 3, 5, 9],
				},
				Bank {
					index: 1,
					scans_per_day: 2,
					registration_time: 2,
					coins: vec![0, 2, 6],
				},
				Bank {
					index: 2,
					scans_per_day: 1,
					registration_time: 2,
					coins: vec![1, 4, 6, 7],
				},
				Bank {
					index: 3,
					scans_per_day: 1,
					registration_time: 2,
					coins: vec![8, 0, 1, 2, 5],
				}
			],
		})
	);
}
