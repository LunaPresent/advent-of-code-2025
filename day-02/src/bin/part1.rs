use std::io::{self, Read as _};

use color_eyre::eyre;

use day_02::*;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;
	let _ = input.pop();

	let mut sum = 0;
	for range_str in input.split(',') {
		let id_range = range_str.parse::<IdRange>()?;
		for range in id_range.split_by_length().filter(|r| r.length() % 2 == 0) {
			for invalid_id in range.ids_with_pattern(2) {
				sum += invalid_id;
			}
		}
	}

	println!("{sum}");

	Ok(())
}
