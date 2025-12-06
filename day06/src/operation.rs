use std::str::FromStr;

use color_eyre::eyre;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
	Add,
	Multiply,
}

impl FromStr for Operation {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(Self::Add),
			"*" => Ok(Self::Multiply),
			_ => Err(eyre::eyre!("invalid operation")),
		}
	}
}

impl Operation {
	pub fn default_value(self) -> u64 {
		match self {
			Self::Add => 0,
			Self::Multiply => 1,
		}
	}

	pub fn execute(self, lhs: u64, rhs: u64) -> u64 {
		match self {
			Self::Add => lhs + rhs,
			Self::Multiply => lhs * rhs,
		}
	}
}
