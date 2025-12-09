use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Circuit {
	nodes: HashSet<usize>,
}

impl Circuit {
	pub fn len(&self) -> usize {
		self.nodes.len()
	}

	pub fn contains(&self, node: usize) -> bool {
		self.nodes.contains(&node)
	}

	pub fn insert(&mut self, node: usize) -> bool {
		self.nodes.insert(node)
	}

	pub fn merge(&mut self, other: Self) {
		self.nodes.extend(other.nodes);
	}
}
