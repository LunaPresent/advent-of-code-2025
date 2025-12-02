use std::io::{self, Read as _};

use color_eyre::eyre;

use day_02::id_range::*;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;
	let _ = input.pop();

	let mut sum = 0;
	for range_str in input.split(',') {
		let id_range = range_str.parse::<IdRange>()?;
		for range in id_range.split_by_length().filter(|r| r.length() % 2 == 0) {
			let mul = 10u64.pow(range.length() as u32 / 2) + 1;
			let min_factor = (range.min_id() - 1) / mul;
			let max_factor = range.max_id() / mul;
			let until_min_factor = min_factor * (min_factor + 1) / 2;
			let until_max_factor = max_factor * (max_factor + 1) / 2;
			let total_factors = until_max_factor.saturating_sub(until_min_factor);
			sum += total_factors * mul;
		}
	}

	println!("{sum}");

	Ok(())
}
