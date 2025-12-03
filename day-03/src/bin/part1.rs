use std::io;

use color_eyre::eyre::{self, OptionExt};

use day_03::*;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut total_joltage: u64 = 0;
	for line in io::stdin().lines() {
		let bank = line?.parse::<Bank>()?;
		let (first_battery_idx, first_battery) = bank
			.batteries()
			.iter()
			.enumerate()
			.take(bank.batteries().len() - 1)
			.rev()
			.max_by_key(|(_, b)| *b)
			.ok_or_eyre("empty bank")?;
		let second_battery = bank
			.batteries()
			.iter()
			.skip(first_battery_idx + 1)
			.max()
			.ok_or_eyre("empty bank")?;
		total_joltage += (10 * first_battery.joltage + second_battery.joltage) as u64;
	}

	println!("{total_joltage}");
	Ok(())
}
