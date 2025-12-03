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
			.map(|c| -> Result<Battery, eyre::Report> { Battery::try_from(c) })
			.collect::<Result<Vec<Battery>, eyre::Report>>()?;
		Ok(Bank { batteries })
	}
}

impl Bank {
	pub fn batteries(&self) -> &[Battery] {
		self.batteries.as_slice()
	}

	pub fn max_joltage(&self, active_battery_count: usize) -> u64 {
		let mut max_joltage = 0;
		let mut head = 0;
		for n in (0..active_battery_count).rev() {
			let (i, joltage) = self
				.batteries()
				.iter()
				.map(|b| b.joltage)
				.enumerate()
				.take(self.batteries().len() - n)
				.skip(head)
				.rev()
				.max_by_key(|(_, j)| *j)
				.unwrap_or((0, 0));
			max_joltage += 10u64.pow(n as u32) * joltage as u64;
			head = i + 1;
		}
		max_joltage
	}
}
