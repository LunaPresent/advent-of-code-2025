use std::io;

use color_eyre::eyre;

use day05::RangeList;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut fresh_list = RangeList::default();
	for line in io::stdin().lines() {
		let line = line?;
		if line.is_empty() {
			break;
		}
		fresh_list.push(line.parse()?);
	}

	let mut fresh_count = 0;
	for line in io::stdin().lines() {
		if fresh_list.contains(line?.parse()?) {
			fresh_count += 1;
		}
	}

	println!("{fresh_count}");
	Ok(())
}
