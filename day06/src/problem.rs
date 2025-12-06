use std::{fmt::Display, iter};

use crate::Operation;

#[derive(Debug)]
pub struct Problem {
	operands: Vec<u64>,
	operation: Operation,
}

impl Problem {
	pub fn new(operand_count: usize, operation: Operation) -> Self {
		Self {
			operands: iter::repeat_n(0, operand_count).collect(),
			operation,
		}
	}

	pub fn operands_mut(&mut self) -> &mut [u64] {
		self.operands.as_mut_slice()
	}

	pub fn solve(&self) -> u64 {
		self.operands
			.iter()
			.map(|&x| x)
			.reduce(|lhs, rhs| self.operation.execute(lhs, rhs))
			.unwrap_or(0)
	}
}

impl Display for Problem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for op in &self.operands {
			write!(f, "{op}, ")?;
		}

		Ok(())
	}
}
