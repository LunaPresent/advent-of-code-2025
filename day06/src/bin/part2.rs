use std::io::{self, Read};

use color_eyre::eyre::{self, OptionExt as _};
use day06::{Operation, Problem};

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut s = String::new();
	io::stdin().read_to_string(&mut s)?;

	let (table, operations) = s[..s.len() - 1]
		.rsplit_once('\n')
		.ok_or_eyre("invalid format")?;

	let mut problems = operations
		.char_indices()
		.rev()
		.filter(|(_, c)| !c.is_ascii_whitespace())
		.scan(
			operations.len() + 1,
			|prev: &mut usize, (i, c)| -> Option<Problem> {
				let count = *prev - i - 1;
				*prev = i;
				Some(Problem::new(count, Operation::try_from(c).ok()?))
			},
		)
		.collect::<Vec<Problem>>();
	problems.reverse();

	for line in table.lines() {
		let mut chars = line.chars();
		for problem in problems.iter_mut() {
			for operand in problem.operands_mut() {
				let c = chars.next().ok_or_eyre("line too short")?;
				if let Some(v) = c.to_digit(10).map(|v| v as u64) {
					*operand = *operand * 10 + v;
				}
			}
			chars.next();
		}
	}

	let grand_total = problems.iter().map(|p| p.solve()).sum::<u64>();
	println!("{grand_total}");
	Ok(())
}
