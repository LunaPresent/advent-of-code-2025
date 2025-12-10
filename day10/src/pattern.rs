use std::str::FromStr;

use color_eyre::eyre;

#[derive(Debug, Clone, Copy)]
pub struct Pattern {
	pub length: usize,
	pub target: usize,
}

impl FromStr for Pattern {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() < 3 {
			Err(eyre::eyre!("invalid pattern format"))?;
		}
		let length = s.len() - 2;
		let target = s[1..s.len() - 1]
			.char_indices()
			.map(|(n, c)| ((c == '#') as usize) << n)
			.sum();
		Ok(Self { length, target })
	}
}
