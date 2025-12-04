use std::io;

use color_eyre::eyre;

use day03::*;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut total_joltage: u64 = 0;
	for line in io::stdin().lines() {
		let bank = line?.parse::<Bank>()?;
		total_joltage += bank.max_joltage(12);
	}

	println!("{total_joltage}");
	Ok(())
}
