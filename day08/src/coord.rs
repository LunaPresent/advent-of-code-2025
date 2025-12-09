use std::str::FromStr;

use color_eyre::eyre::{self, OptionExt};

#[derive(Debug, Clone, Copy)]
pub struct Coord {
	pub x: u32,
	pub y: u32,
	pub z: u32,
}

impl FromStr for Coord {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, yz) = s.split_once(',').ok_or_eyre("invalid coord")?;
		let (y, z) = yz.split_once(',').ok_or_eyre("invalid coord")?;
		Ok(Self {
			x: x.parse()?,
			y: y.parse()?,
			z: z.parse()?,
		})
	}
}

impl Coord {
	pub fn square_distance(self, other: Coord) -> u64 {
		let dx = self.x as i64 - other.x as i64;
		let dy = self.y as i64 - other.y as i64;
		let dz = self.z as i64 - other.z as i64;
		(dx * dx + dy * dy + dz * dz) as u64
	}
}
