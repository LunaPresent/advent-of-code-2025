use color_eyre::eyre;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollSlot {
	Empty,
	Filled,
}

impl TryFrom<char> for RollSlot {
	type Error = eyre::Report;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'.' => Ok(Self::Empty),
			'@' => Ok(Self::Filled),
			c => Err(eyre::eyre!("invalid character: {c}")),
		}
	}
}
