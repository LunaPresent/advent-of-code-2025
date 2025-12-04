use std::io;

use color_eyre::eyre;

use day04::*;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut roll_stack = RollStack::default();
	for line in io::stdin().lines() {
		let layer = line?
			.chars()
			.map(|c| RollSlot::try_from(c))
			.collect::<eyre::Result<Vec<_>>>()?;
		roll_stack.add_layer(layer);
	}

	let moveable_count = roll_stack
		.slots()
		.iter()
		.enumerate()
		.filter(|(i, slot)| {
			**slot == RollSlot::Filled
				&& roll_stack
					.neighbours(*i)
					.filter(|n| *n == RollSlot::Filled)
					.count() < 4
		})
		.count();

	println!("{moveable_count}");
	Ok(())
}
