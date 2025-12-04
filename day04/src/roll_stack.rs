use crate::RollSlot;

#[derive(Debug, Default)]
pub struct RollStack {
	slots: Vec<RollSlot>,
	width: usize,
}

impl RollStack {
	pub fn add_layer(&mut self, layer: impl IntoIterator<Item = RollSlot>) {
		let len = self.slots.len();
		self.slots.extend(layer);
		// if the width is ever variable this is hella wrong but who cares
		self.width = self.slots.len() - len;
	}

	pub fn slots(&self) -> &[RollSlot] {
		self.slots.as_slice()
	}

	pub fn neighbours(&self, slot_idx: usize) -> impl Iterator<Item = RollSlot> {
		let width = self.width as isize;
		let height = self.slots.len() as isize / width;
		let x = slot_idx as isize % width;
		let y = slot_idx as isize / width;

		const OFFSETS: [(isize, isize); 8] = [
			(-1, -1),
			(0, -1),
			(1, -1),
			(-1, 0),
			(1, 0),
			(-1, 1),
			(0, 1),
			(1, 1),
		];

		OFFSETS.iter().filter_map(move |(xoff, yoff)| {
			let x = x + xoff;
			let y = y + yoff;
			((0..width).contains(&x) && (0..height).contains(&y))
				.then(|| self.slots[(x + y * width) as usize])
		})
	}
}
