use itertools::Itertools;

pub type CoinValue = u16;
pub type CoinIndex = u32;
pub type BankIndex = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bank {
	pub index: BankIndex,
	pub registration_time: u32,
	pub scans_per_day: u32,
	pub coins: Vec<CoinIndex>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Challenge {
	pub coins: Vec<u16>,
	pub banks: Vec<Bank>,
	pub days_to_scan: u32,
}

pub fn parse(data: &str) -> Challenge {
	let mut lines = data.lines();
	let (amount_coins, amount_banks, days_to_scan) = lines
		.next()
		.expect("First line missing.")
		.split(' ')
		.map(|n| n.parse::<u32>().expect("Failed to parse header line."))
		.collect_tuple()
		.expect("First lined does not contain 3 values.");

	let coins = lines
		.next()
		.expect("No coins found.")
		.split(' ')
		.map(|n| n.parse::<CoinValue>().expect("Failed to parse coin value."))
		.collect_vec();
	assert_eq!(
		coins.len() as u32,
		amount_coins,
		"Coin length does not match."
	);

	let banks = lines
		.tuples()
		.enumerate()
		.map(|(i, (f, s))| {
			let (amount_coins, registration_time, scans_per_day) = f
				.split(' ')
				.map(|n| n.parse::<u32>().expect("Bank header line can't be parsed."))
				.collect_tuple()
				.expect("Bank header line does not contain 3 values.");
			let coins = s
				.split(' ')
				.map(|n| n.parse::<CoinIndex>().expect("Failed to parse coin value."))
				.collect_vec();
			assert_eq!(
				coins.len() as u32,
				amount_coins,
				"Coin length does not match."
			);

			Bank {
				index: i as BankIndex,
				coins,
				registration_time,
				scans_per_day,
			}
		})
		.collect_vec();

	assert_eq!(
		banks.len() as u32,
		amount_banks,
		"Bank length does not match."
	);

	Challenge {
		banks,
		coins,
		days_to_scan,
	}
}
