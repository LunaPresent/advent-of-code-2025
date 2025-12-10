use std::iter;
use std::str::FromStr;

use color_eyre::eyre::{self, OptionExt as _};

use crate::button::Button;
use crate::pattern::Pattern;

#[derive(Debug)]
pub struct Schematic {
	pattern: Pattern,
	buttons: Vec<Button>,
}

impl FromStr for Schematic {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (pattern, s) = s.split_once(' ').ok_or_eyre("invalid format")?;
		let (buttons, _joltages) = s.rsplit_once(' ').ok_or_eyre("invalid format")?;
		let pattern = pattern.parse::<Pattern>()?;
		let buttons = buttons
			.split(' ')
			.map(|s| -> eyre::Result<_> { s.parse::<Button>() })
			.collect::<eyre::Result<_>>()?;
		Ok(Self { pattern, buttons })
	}
}

impl Schematic {
	pub fn shortest_distance(&self) -> usize {
		let mut distances =
			iter::repeat_n(usize::MAX - 1, 1 << self.pattern.length).collect::<Vec<_>>();
		distances[0] = 0;

		loop {
			let (n, &d) = distances.iter().enumerate().min_by_key(|x| x.1).unwrap();

			if n == self.pattern.target {
				break d;
			}

			for button in &self.buttons {
				let i = n ^ button.transition;
				if distances[i] != usize::MAX {
					distances[i] = distances[i].min(d + 1);
				}
			}

			distances[n] = usize::MAX;
		}
	}
}
