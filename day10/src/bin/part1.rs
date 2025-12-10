use std::io;

use color_eyre::eyre;

use day10::Schematic;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut sum = 0;
	for line in io::stdin().lines() {
		let schematic = line?.parse::<Schematic>()?;
		let d = schematic.shortest_distance();
		sum += d;
	}

	println!("{sum}");
	Ok(())
}
