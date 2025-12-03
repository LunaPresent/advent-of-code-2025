use std::io::{self, Read as _};

use color_eyre::eyre;

use day_02::*;

fn next_lowest_id(id_iters: &mut Vec<InvalidIdIterator>) -> Option<u64> {
	id_iters
		.iter_mut()
		.filter(|iter| iter.peek().is_some())
		.min_by_key(|iter| iter.peek())?
		.next()
}

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;
	let _ = input.pop();

	let mut sum = 0;
	for range_str in input.split(',') {
		let id_range = range_str.parse::<IdRange>()?;
		for range in id_range.split_by_length() {
			let mut invalid_id_iterators = (2..=range.length())
				.filter(|n| range.length() % n == 0)
				.map(|f| range.ids_with_pattern(f))
				.collect::<Vec<_>>();
			let mut prev_id = 0;
			while let Some(id) = next_lowest_id(&mut invalid_id_iterators) {
				if id == prev_id {
					continue;
				}
				sum += id;
				prev_id = id;
			}
		}
	}

	println!("{sum}");

	Ok(())
}
