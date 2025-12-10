use std::io::{self, Read as _};

use color_eyre::eyre;
use day09::Floor;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut s = String::new();
	io::stdin().read_to_string(&mut s)?;

	let floor = s.parse::<Floor>()?;

	let max_area = floor.extreme_rects().map(|r| r.area()).max().unwrap();

	println!("{max_area}");

	Ok(())
}
