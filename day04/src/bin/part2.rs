use std::io;

use color_eyre::eyre;

use day04::*;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut total_removed = 0;
	let mut roll_stack = RollStack::default();
	for line in io::stdin().lines() {
		let layer = line?
			.chars()
			.map(|c| RollSlot::try_from(c))
			.collect::<eyre::Result<Vec<_>>>()?;
		roll_stack.add_layer(layer);
	}

	loop {
		let mut removed = 0;
		for i in 0..roll_stack.slots().len() {
			let slot = roll_stack.slots()[i];
			if slot == RollSlot::Empty
				|| roll_stack
					.neighbours(i)
					.filter(|n| *n != RollSlot::Empty)
					.count() >= 4
			{
				continue;
			}

			roll_stack.slots_mut()[i] = RollSlot::Removeable;
			removed += 1;
		}

		if removed == 0 {
			break;
		}

		roll_stack
			.slots_mut()
			.iter_mut()
			.filter(|slot| **slot == RollSlot::Removeable)
			.for_each(|slot| *slot = RollSlot::Empty);

		total_removed += removed;
	}

	println!("{total_removed}");
	Ok(())
}
