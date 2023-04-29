use std::collections::HashSet;

use itertools::Itertools;

use crate::output::Output;
use crate::parser::Challenge;

use super::Algorithm;

pub struct Idee1;

impl Algorithm for Idee1 {
	fn run(challenge: Challenge) -> Output {
		let mut total_coins = HashSet::new();
		let coin_frequency = challenge
			.banks
			.iter()
			.flat_map(|b| b.coins.iter().copied())
			.counts();
		let coin_worth = challenge
			.coins
			.iter()
			.enumerate()
			.map(|(index, &worth)| worth as f32 / coin_frequency[&(index as u32)] as f32)
			.collect_vec();

		let bank_order = challenge
			.banks
			.iter()
			.sorted_by_cached_key(|b| {
				(challenge.days_to_scan - b.registration_time) * b.coins.len() as u32
			})
			.collect_vec();

		let mut registrations = Vec::new();
		let mut total_days = 0;
		for bank in bank_order {
			total_days += bank.registration_time;
			let Some(days_left) = challenge.days_to_scan.checked_sub(total_days) else {break;};
			let max_coins_to_scan = days_left * bank.scans_per_day;
			let coins = bank
				.coins
				.iter()
				.copied()
				.sorted_by(|a, b| coin_worth[*b as usize].total_cmp(&coin_worth[*a as usize]))
				.take(max_coins_to_scan as usize)
				.collect_vec();
			total_coins.extend(coins.iter().copied());
			registrations.push((bank.clone(), coins));
		}
		println!(
			"Total worth: {}",
			total_coins
				.iter()
				.map(|i| challenge.coins[*i as usize] as u32)
				.sum::<u32>()
		);
		Output {
			registrations,
			coins: total_coins,
		}
	}
}
