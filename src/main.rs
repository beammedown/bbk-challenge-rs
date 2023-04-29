#![feature(int_roundings)]

use crate::algorithm::Algorithm;
use crate::parser::{parse, Bank, Challenge};

pub mod algorithm;
pub mod output;
pub mod parser;

fn main() {
	let handle_a = std::thread::spawn(|| {
		println!(
			"a: {}",
			Algorithm::execute(parse(include_str!("../challenges/a_example.txt"))).total_value
		)
	});
	let handle_b = std::thread::spawn(|| {
		println!(
			"b: {}",
			Algorithm::execute(parse(include_str!("../challenges/b_read_on.txt"))).total_value
		)
	});
	let handle_c = std::thread::spawn(|| {
		println!(
			"c: {}",
			Algorithm::execute(parse(include_str!("../challenges/c_incunabula.txt"))).total_value
		)
	});
	let handle_d = std::thread::spawn(|| {
		println!(
			"d: {}",
			Algorithm::execute(parse(include_str!("../challenges/d_tough_choices.txt")))
				.total_value
		)
	});
	let handle_e = std::thread::spawn(|| {
		println!(
			"e: {}",
			Algorithm::execute(parse(include_str!("../challenges/e_so_many_coins.txt")))
				.total_value
		)
	});
	let handle_f = std::thread::spawn(|| {
		println!(
			"f: {}",
			Algorithm::execute(parse(include_str!(
				"../challenges/f_banks_of_the_world.txt"
			)))
			.total_value
		)
	});
	for handle in [handle_a, handle_b, handle_c, handle_d, handle_e, handle_f] {
		handle.join().unwrap();
	}
	// println!(
	// 	"{}",
	// 	Algorithm::run(Challenge {
	// 		days_to_scan: 7,
	// 		coins: vec![12, 5, 8, 10, 9, 3, 2, 1, 4, 6],
	// 		banks: vec![
	// 			Bank {
	// 				index: 0,
	// 				scans_per_day: 1,
	// 				registration_time: 2,
	// 				coins: vec![0, 2, 3, 5, 9],
	// 			},
	// 			Bank {
	// 				index: 1,
	// 				scans_per_day: 2,
	// 				registration_time: 2,
	// 				coins: vec![0, 2, 6],
	// 			},
	// 			Bank {
	// 				index: 2,
	// 				scans_per_day: 1,
	// 				registration_time: 2,
	// 				coins: vec![1, 4, 6, 7],
	// 			},
	// 			Bank {
	// 				index: 3,
	// 				scans_per_day: 1,
	// 				registration_time: 2,
	// 				coins: vec![8, 0, 1, 2, 5],
	// 			}
	// 		],
	// 	})
	// );
	// println!(
	// 	"{:#?}",
	// 	Algorithm::run(Challenge {
	// 		days_to_scan: 9,
	// 		coins: vec![10, 14, 16, 2, 4, 12, 18, 6, 8, 20],
	// 		banks: vec![
	// 			Bank {
	// 				index: 0,
	// 				scans_per_day: 1,
	// 				registration_time: 2,
	// 				coins: vec![2, 3, 5, 7],
	// 			},
	// 			Bank {
	// 				index: 1,
	// 				scans_per_day: 2,
	// 				registration_time: 3,
	// 				coins: vec![0, 1, 2, 3, 4],
	// 			},
	// 			Bank {
	// 				index: 2,
	// 				scans_per_day: 1,
	// 				registration_time: 2,
	// 				coins: vec![2, 9],
	// 			},
	// 			Bank {
	// 				index: 3,
	// 				scans_per_day: 2,
	// 				registration_time: 2,
	// 				coins: vec![0, 4, 6, 8],
	// 			},
	// 			Bank {
	// 				index: 4,
	// 				scans_per_day: 2,
	// 				registration_time: 2,
	// 				coins: vec![1, 2, 6, 7, 9],
	// 			},
	// 			Bank {
	// 				index: 5,
	// 				scans_per_day: 2,
	// 				registration_time: 3,
	// 				coins: vec![5, 6, 7, 8],
	// 			}
	// 		],
	// 	})
	// );
}

// fn main() {
// 	println!(
// 		"d: {}",
// 		Algorithm::run(parse(include_str!("../challenges/d_tough_choices.txt"))).
// total_value 	)
// }
