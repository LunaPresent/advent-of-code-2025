use std::str::FromStr;

use color_eyre::eyre::{self, OptionExt};

#[derive(Debug, Clone, Copy)]
pub struct IdRange {
	from: usize,
	to: usize,
}

impl FromStr for IdRange {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (from, to) = s.split_once('-').ok_or_eyre("missing hypen")?;
		let from = from.parse()?;
		let to = to.parse::<usize>()? + 1;
		Ok(Self { from, to })
	}
}

impl IdRange {
	pub fn len(self) -> usize {
		self.to - self.from
	}

	pub fn contains(self, id: usize) -> bool {
		id >= self.from && id < self.to
	}

	pub fn union(self, other: IdRange) -> Option<IdRange> {
		if self.from > other.to || other.from > self.to {
			return None;
		}

		Some(IdRange {
			from: self.from.min(other.from),
			to: self.to.max(other.to),
		})
	}
}
