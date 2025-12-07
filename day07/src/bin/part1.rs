use std::{io, iter, mem};

use color_eyre::eyre::{self, OptionExt};

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut lines = io::stdin().lines();

	let mut beams = lines
		.next()
		.ok_or_eyre("input empty")??
		.chars()
		.map(|c| c == 'S')
		.collect::<Vec<_>>();
	let mut beams_next = iter::repeat_n(false, beams.len()).collect::<Vec<_>>();
	let mut split_count = 0;

	for line in lines {
		for (i, is_splitter) in beams
			.iter()
			.zip(line?.chars().map(|c| c == '^'))
			.enumerate()
			.filter_map(|(i, (b, s))| b.then_some((i, s)))
		{
			if !is_splitter {
				beams_next[i] = true;
				continue;
			}

			split_count += 1;
			beams_next[i - 1] = true;
			beams_next[i + 1] = true;
		}
		mem::swap(&mut beams, &mut beams_next);
		beams_next.fill(false);
	}

	println!("{split_count}");
	Ok(())
}
