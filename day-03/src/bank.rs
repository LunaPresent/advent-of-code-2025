use std::str::FromStr;

use color_eyre::eyre::{self};

use crate::battery::Battery;

#[derive(Debug)]
pub struct Bank {
	batteries: Vec<Battery>,
}

impl FromStr for Bank {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let batteries = s
			.chars()
			.map(|c| -> Result<Battery, char> {
				Ok(Battery {
					joltage: c.to_digit(10).ok_or(c)? as u8,
				})
			})
			.collect::<Result<Vec<Battery>, char>>()
			.map_err(|c| eyre::eyre!("invalid digit: {c}"))?;
		Ok(Bank { batteries })
	}
}

impl Bank {
	pub fn batteries(&self) -> &[Battery] {
		self.batteries.as_slice()
	}
}
