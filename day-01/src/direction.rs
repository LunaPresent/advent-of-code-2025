use color_eyre::eyre;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Left,
	Right,
}

impl TryFrom<char> for Direction {
	type Error = eyre::Report;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'L' => Ok(Direction::Left),
			'R' => Ok(Direction::Right),
			_ => Err(eyre::eyre!("invalid char to Direction: {value}")),
		}
	}
}
