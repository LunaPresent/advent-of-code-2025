use std::str::FromStr;

use color_eyre::eyre::{self, OptionExt as _};

#[derive(Debug, Clone, Copy)]
pub struct IdRange {
	min_id: u64,
	max_id: u64,
}

impl IdRange {
	pub fn split_by_length(self) -> IdRangeSplitByLength {
		IdRangeSplitByLength {
			min_id_remaining: self.min_id,
			max_id: self.max_id,
		}
	}
}

impl FromStr for IdRange {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (min_id_str, max_id_str) = s.split_once('-').ok_or_eyre("missing hypen in input")?;
		let min_id = min_id_str.parse()?;
		let max_id = max_id_str.parse()?;
		Ok(IdRange { min_id, max_id })
	}
}

#[derive(Debug, Clone, Copy)]
pub struct FixedLengthIdRange {
	min_id: u64,
	max_id: u64,
	length: usize,
}

impl FixedLengthIdRange {
	pub fn min_id(self) -> u64 {
		self.min_id
	}

	pub fn max_id(self) -> u64 {
		self.max_id
	}

	pub fn length(self) -> usize {
		self.length
	}

	pub fn ids_with_pattern(self, pattern_count: usize) -> InvalidIdIterator {
		let pattern_length = self.length / pattern_count;
		let value_to_next: u64 = (0..pattern_count)
			.map(|n| 10u64.pow((n * pattern_length) as u32))
			.sum();
		let next_id = ((self.min_id - 1) / value_to_next + 1) * value_to_next;

		InvalidIdIterator {
			next_id,
			max_id: self.max_id,
			value_to_next,
		}
	}
}

#[derive(Debug)]
pub struct IdRangeSplitByLength {
	min_id_remaining: u64,
	max_id: u64,
}

impl Iterator for IdRangeSplitByLength {
	type Item = FixedLengthIdRange;

	fn next(&mut self) -> Option<Self::Item> {
		if self.min_id_remaining > self.max_id {
			return None;
		}

		let length = self.min_id_remaining.max(1).ilog10() + 1;
		let next_div_10 = 10u64.pow(length);
		let next = FixedLengthIdRange {
			min_id: self.min_id_remaining,
			max_id: self.max_id.min(next_div_10 - 1),
			length: length as usize,
		};
		self.min_id_remaining = next.max_id + 1;

		Some(next)
	}
}

#[derive(Debug)]
pub struct InvalidIdIterator {
	next_id: u64,
	max_id: u64,
	value_to_next: u64,
}

impl InvalidIdIterator {
	pub fn peek(&self) -> Option<u64> {
		(self.next_id <= self.max_id).then_some(self.next_id)
	}
}

impl Iterator for InvalidIdIterator {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.next_id > self.max_id {
			return None;
		}
		let id = self.next_id;
		self.next_id += self.value_to_next;
		Some(id)
	}
}
