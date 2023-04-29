use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

use crate::parser::{BankIndex, CoinIndex};

#[derive(Debug)]
pub struct Output {
	pub registrations: Vec<(BankIndex, Vec<CoinIndex>)>,
	pub coins: HashSet<CoinIndex>,
	pub total_value: u32,
}
impl Display for Output {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.registrations.len())?;
		for (bank, coins) in &self.registrations {
			writeln!(f, "{} {}", bank, coins.len())?;
			writeln!(f, "{}", coins.iter().map(|c| c.to_string()).join(" "))?;
		}
		Ok(())
	}
}
