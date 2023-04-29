use std::collections::HashSet;

use crate::parser::{Bank, CoinIndex};

#[derive(Debug)]
pub struct Output {
	pub registrations: Vec<(Bank, Vec<CoinIndex>)>,
	pub coins: HashSet<CoinIndex>,
}
