use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::output::Output;
use crate::parser::Challenge;

pub struct Algorithm;

impl Algorithm {
	pub fn execute(challenge: Challenge) -> Output {
		let mut total_coins = HashSet::new();
		let mut total_days = 0;
		let mut registrations = Vec::new();
		let mut unregistrated_banks = challenge.banks.clone();

		let initial_coin_frequency = unregistrated_banks
			.iter()
			.flat_map(|b| b.coins.iter().copied())
			.counts();

		let mut coin_worth = challenge
			.coins
			.iter()
			.enumerate()
			.map(|(index, &worth)| {
				initial_coin_frequency
					.get(&(index as u32))
					.map(|f| worth as f32 / (*f as f32 + 1.0))
					.unwrap_or(0.0)
			})
			.collect_vec();

		// let mut i = 0;
		while total_days < challenge.days_to_scan {
			// i += 1;
			// if i % 100 == 0 {
			// 	print!(
			// 		"\r|{}>{}|  {}/{}",
			// 		"=".repeat((total_days * 20 / challenge.days_to_scan) as usize),
			// 		" ".repeat((20 - total_days * 20 / challenge.days_to_scan) as usize),
			// 		total_days,
			// 		challenge.days_to_scan
			// 	);
			// 	std::io::Write::flush(&mut std::io::stdout()).unwrap();
			// }

			let days_left = challenge.days_to_scan - total_days;

			let Some(((i, bank, useful_coins), _)) = unregistrated_banks
				.par_iter()
				.enumerate()
				.filter_map(|(i, b)| {
					days_left.checked_sub(b.registration_time).map(|d| {
						let mut useful_coins = b
							.coins
							.iter()
							.filter(|c| !total_coins.contains(*c))
							.map(|c| (*c, coin_worth[*c as usize]))
							.sorted_by(|(_, a), (_, b)| b.total_cmp(a))
							.map(|(i, _)| i)
							.collect_vec();
						let max_scanned_coins =
							(b.scans_per_day * d).min(useful_coins.len() as u32);
						let days_working = max_scanned_coins.div_ceil(b.scans_per_day).min(d);
						useful_coins.truncate(max_scanned_coins as usize);
						let score = days_working as f32
							* useful_coins
								.iter()
								.map(|c| coin_worth[*c as usize])
								.sum::<f32>();
						((i, b, useful_coins), score)
					})
				}).max_by(|(_, a), (_, b)| a.total_cmp(b)) else {break;};

			total_days += bank.registration_time;

			total_coins.extend(useful_coins.iter().copied());
			for coin in &useful_coins {
				coin_worth[*coin as usize] = 0.0;
			}
			for coin in &bank.coins {
				if !useful_coins.contains(&coin) {
					coin_worth[*coin as usize] = challenge.coins[*coin as usize] as f32
						/ (challenge.coins[*coin as usize] as f32 / coin_worth[*coin as usize]
							- 1.0);
				}
			}
			registrations.push((bank.index, useful_coins));
			unregistrated_banks.swap_remove(i);
		}
		Output {
			registrations,
			total_value: total_coins
				.iter()
				.map(|i| challenge.coins[*i as usize] as u32)
				.sum::<u32>(),
			coins: total_coins,
		}
	}
}
