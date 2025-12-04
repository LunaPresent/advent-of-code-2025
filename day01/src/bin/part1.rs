use std::io;

use color_eyre::eyre;

use day01::safe::Safe;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let mut safe = Safe::new(0, 99);
	safe.set_code(50)?;

	let mut count = 0;
	for line in io::stdin().lines() {
		let movement = line?.parse()?;
		safe.turn(movement);
		if safe.current_code() == 0 {
			count += 1;
		}
	}

	println!("{count}");

	Ok(())
}
