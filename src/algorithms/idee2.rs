use std::collections::HashSet;

use itertools::Itertools;

use crate::output::Output;
use crate::parser::Challenge;

use super::Algorithm;

pub struct Idee2;

impl Algorithm for Idee2 {
	fn run(challenge: Challenge) -> Output {
		let mut total_coins = HashSet::new();
		let mut total_days = 0;
		let mut registrations = Vec::new();
		let mut unregistrated_banks = challenge.banks.clone();
		while total_days < challenge.days_to_scan {
			let days_left = challenge.days_to_scan - total_days;
			let coin_frequency = unregistrated_banks
				.iter()
				.flat_map(|b| {
					b.coins
						.iter()
						.filter(|c| !total_coins.contains(*c))
						.copied()
				})
				.counts();
			let coin_worth = challenge
				.coins
				.iter()
				.enumerate()
				.map(|(index, &worth)| {
					coin_frequency
						.get(&(index as u32))
						.map(|f| worth as f32 / *f as f32)
						.unwrap_or(0.0)
				})
				.collect_vec();

			let bank_order = unregistrated_banks
				.iter()
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
								.sum::<f32>() * b.scans_per_day as f32;
						((i, b, useful_coins), score)
					})
				})
				.sorted_by_key(|(_, e)| -(*e as i32))
				.map(|(b, _)| b)
				.collect_vec();

			let Some((i, bank, useful_coins)) = bank_order.first() else {break;};
			total_days += bank.registration_time;

			total_coins.extend(useful_coins.iter().copied());
			registrations.push(((*bank).clone(), useful_coins.clone()));
			unregistrated_banks.remove(*i);
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
