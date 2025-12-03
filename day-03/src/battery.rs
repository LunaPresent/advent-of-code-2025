use color_eyre::eyre::{self, OptionExt as _};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Battery {
	pub joltage: u8,
}

impl TryFrom<char> for Battery {
	type Error = eyre::Report;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(Self {
			joltage: value
				.to_digit(10)
				.ok_or_eyre(format!("invalid digit: {value}"))? as u8,
		})
	}
}
