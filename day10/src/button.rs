use std::str::FromStr;

use color_eyre::eyre;

#[derive(Debug, Clone, Copy)]
pub struct Button {
	pub transition: usize,
}

impl FromStr for Button {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() < 3 {
			Err(eyre::eyre!("invalid button format"))?;
		}
		let mut transition = 0;
		for s in s[1..s.len() - 1].split(',') {
			transition |= 1 << s.parse::<usize>()?;
		}
		Ok(Self { transition })
	}
}
