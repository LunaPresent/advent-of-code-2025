#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DistancePair {
	pub square_distance: u64,
	pub first: usize,
	pub second: usize,
}
