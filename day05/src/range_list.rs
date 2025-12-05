use crate::id_range::IdRange;

#[derive(Debug, Default)]
pub struct RangeList {
	ranges: Vec<IdRange>,
}

impl RangeList {
	pub fn push(&mut self, mut range: IdRange) {
		let mut i = 0;
		while i < self.ranges.len() {
			if let Some(union) = range.union(self.ranges[i]) {
				self.ranges.swap_remove(i);
				range = union;
			} else {
				i += 1;
			}
		}
		self.ranges.push(range)
	}

	pub fn contains(&self, id: usize) -> bool {
		self.ranges.iter().any(|range| range.contains(id))
	}
}
