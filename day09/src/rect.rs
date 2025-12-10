use crate::Coord;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
	min_coord: Coord,
	max_coord: Coord,
}

impl Rect {
	pub fn new(first: Coord, second: Coord) -> Self {
		Self {
			min_coord: Coord {
				x: first.x.min(second.x),
				y: first.y.min(second.y),
			},
			max_coord: Coord {
				x: first.x.max(second.x),
				y: first.y.max(second.y),
			},
		}
	}

	pub fn area(self) -> u64 {
		let dx = self.max_coord.x - self.min_coord.x + 1;
		let dy = self.max_coord.y - self.min_coord.y + 1;
		dx as u64 * dy as u64
	}
}
