use std::{io, iter, mem};

use color_eyre::eyre::{self, OptionExt};

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut lines = io::stdin().lines();

	let mut timelines = lines
		.next()
		.ok_or_eyre("input empty")??
		.chars()
		.map(|c| if c == 'S' { 1 } else { 0 })
		.collect::<Vec<_>>();
	let mut timelines_next = iter::repeat_n(0, timelines.len()).collect::<Vec<_>>();

	for line in lines {
		for (i, timeline_count, is_splitter) in timelines
			.iter()
			.zip(line?.chars().map(|c| c == '^'))
			.enumerate()
			.filter_map(|(i, (t, s))| (*t > 0).then_some((i, *t, s)))
		{
			if !is_splitter {
				timelines_next[i] += timeline_count;
				continue;
			}

			timelines_next[i - 1] += timeline_count;
			timelines_next[i + 1] += timeline_count;
		}
		mem::swap(&mut timelines, &mut timelines_next);
		timelines_next.fill(0);
	}

	println!("{}", timelines.iter().sum::<u64>());
	Ok(())
}
