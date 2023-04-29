use std::collections::HashSet;

use crate::parser::{Bank, CoinIndex};

#[derive(Debug)]
pub struct Output {
	pub registrations: Vec<(u32, Vec<CoinIndex>)>,
	pub total_value: u32,
	pub coins: HashSet<CoinIndex>,
	pub maxscore: u32,
}
