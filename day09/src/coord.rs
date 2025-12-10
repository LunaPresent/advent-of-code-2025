use std::str::FromStr;

use color_eyre::eyre::{self, OptionExt as _};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
	pub x: u32,
	pub y: u32,
}

impl FromStr for Coord {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(',').ok_or_eyre("invalid coord")?;
		Ok(Self {
			x: x.parse()?,
			y: y.parse()?,
		})
	}
}
