use std::io::{self, Read};

use color_eyre::eyre::{self, OptionExt as _};
use day06::Operation;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut s = String::new();
	io::stdin().read_to_string(&mut s)?;

	let (table, operations) = s
		.trim_end()
		.rsplit_once('\n')
		.ok_or_eyre("invalid format")?;
	let operations = operations
		.split_whitespace()
		.map(|s| s.parse::<Operation>())
		.collect::<Result<Vec<_>, _>>()?;
	let mut solutions = operations
		.iter()
		.map(|op| op.default_value())
		.collect::<Vec<u64>>();
	for line in table.lines() {
		line.split_whitespace()
			.map(|s| s.parse::<u64>().unwrap())
			.zip(solutions.iter_mut())
			.zip(operations.iter())
			.for_each(|((lhs, rhs), op)| *rhs = op.execute(lhs, *rhs));
	}

	let grand_total = solutions.iter().sum::<u64>();

	println!("{grand_total}");
	Ok(())
}
