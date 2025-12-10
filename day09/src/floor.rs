use std::str::FromStr;

use color_eyre::eyre;

use crate::{Coord, Rect};

#[derive(Debug, Default)]
pub struct Floor {
	top_lefts: Vec<Coord>,
	top_rights: Vec<Coord>,
	bottom_lefts: Vec<Coord>,
	bottom_rights: Vec<Coord>,
}

impl FromStr for Floor {
	type Err = eyre::Report;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut coords = Vec::new();
		let mut min_x = u32::MAX;
		let mut min_y = u32::MAX;
		let mut max_x = 0;
		let mut max_y = 0;
		for line in s.lines() {
			let coord = line.parse::<Coord>()?;
			min_x = min_x.min(coord.x);
			max_x = max_x.max(coord.x);
			min_y = min_y.min(coord.y);
			max_y = max_y.max(coord.y);
			coords.push(coord);
		}

		if coords.len() == 0 {
			return Ok(Self::default());
		}

		let mut top_lefts = Vec::new();
		top_lefts.push(
			coords
				.iter()
				.filter_map(|c| (c.y == min_y).then_some(*c))
				.min_by_key(|c| c.x)
				.expect("this should always yield"),
		);
		if let Some(left_top) = coords
			.iter()
			.filter_map(|c| (c.x == min_x).then_some(*c))
			.min_by_key(|c| c.y)
			&& left_top != *top_lefts.first().unwrap()
		{
			top_lefts.push(left_top);
			top_lefts.extend(points_inbetween(
				top_lefts[0],
				top_lefts[1],
				coords.as_slice(),
			));
		}

		let mut top_rights = Vec::new();
		top_rights.push(
			coords
				.iter()
				.filter_map(|c| (c.y == min_y).then_some(*c))
				.max_by_key(|c| c.x)
				.expect("this should always yield"),
		);
		if let Some(right_top) = coords
			.iter()
			.filter_map(|c| (c.x == max_x).then_some(*c))
			.min_by_key(|c| c.y)
			&& right_top != *top_rights.first().unwrap()
		{
			top_rights.push(right_top);
			top_rights.extend(points_inbetween(
				top_rights[0],
				top_rights[1],
				coords.as_slice(),
			));
		}

		let mut bottom_lefts = Vec::new();
		bottom_lefts.push(
			coords
				.iter()
				.filter_map(|c| (c.y == max_y).then_some(*c))
				.min_by_key(|c| c.x)
				.expect("this should always yield"),
		);
		if let Some(left_bottom) = coords
			.iter()
			.filter_map(|c| (c.x == min_x).then_some(*c))
			.max_by_key(|c| c.y)
			&& left_bottom != *bottom_lefts.first().unwrap()
		{
			bottom_lefts.push(left_bottom);
			bottom_lefts.extend(points_inbetween(
				bottom_lefts[0],
				bottom_lefts[1],
				coords.as_slice(),
			));
		}

		let mut bottom_rights = Vec::new();
		bottom_rights.push(
			coords
				.iter()
				.filter_map(|c| (c.y == max_y).then_some(*c))
				.max_by_key(|c| c.x)
				.expect("this should always yield"),
		);
		if let Some(right_bottom) = coords
			.iter()
			.filter_map(|c| (c.x == max_x).then_some(*c))
			.max_by_key(|c| c.y)
			&& right_bottom != *bottom_rights.first().unwrap()
		{
			bottom_rights.push(right_bottom);
			bottom_rights.extend(points_inbetween(
				bottom_rights[0],
				bottom_rights[1],
				coords.as_slice(),
			));
		}

		Ok(Self {
			top_lefts,
			top_rights,
			bottom_rights,
			bottom_lefts,
		})
	}
}

fn points_inbetween(first: Coord, second: Coord, coords: &[Coord]) -> impl Iterator<Item = Coord> {
	coords.iter().filter_map(move |c| {
		(c.x > first.x.min(second.x)
			&& c.x < first.x.max(second.x)
			&& c.y > first.y.min(second.y)
			&& c.y < first.y.max(second.y))
		.then_some(*c)
	})
}

impl Floor {
	pub fn extreme_rects(&self) -> impl Iterator<Item = Rect> {
		let tl_to_br = self
			.top_lefts
			.iter()
			.flat_map(|&tl| self.bottom_rights.iter().map(move |&br| Rect::new(tl, br)));
		let tr_to_bl = self
			.top_rights
			.iter()
			.flat_map(|&tr| self.bottom_lefts.iter().map(move |&bl| Rect::new(tr, bl)));

		tl_to_br.chain(tr_to_bl)
	}
}
