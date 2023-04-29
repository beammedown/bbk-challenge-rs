use crate::output::Output;
use crate::parser::Challenge;

// pub mod idee1;
pub mod idee2;

pub trait Algorithm {
	fn run(challenge: Challenge) -> Output;
}
