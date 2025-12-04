use std::str::FromStr;

use color_eyre::eyre::{self, OptionExt as _};

use crate::direction::Direction;

#[derive(Debug, Clone, Copy)]
pub struct Movement {
	pub direction: Direction,
	pub amount: u16,
}

impl FromStr for Movement {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let first_char = s.chars().next().ok_or_eyre("empty string")?;
		let direction = Direction::try_from(first_char)?;
		let amount = s[first_char.len_utf8()..].parse()?;
		Ok(Movement { direction, amount })
	}
}
