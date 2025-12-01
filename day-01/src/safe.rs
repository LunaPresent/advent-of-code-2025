use color_eyre::eyre;

use crate::{direction::Direction, movement::Movement};

#[derive(Debug, Clone, Copy)]
pub struct Safe {
	min_code: i32,
	max_code: i32,
	current_code: i32,
}

impl Safe {
	pub fn new(min_code: i32, max_code: i32) -> Self {
		assert!(min_code < max_code);
		Self {
			min_code,
			max_code,
			current_code: min_code,
		}
	}

	pub fn set_code(&mut self, value: i32) -> eyre::Result<()> {
		if value < self.min_code || value > self.max_code {
			return Err(eyre::eyre!(
				"set_code: value should be between {} and {}",
				self.min_code,
				self.max_code
			));
		}
		self.current_code = value;
		Ok(())
	}

	pub fn turn(&mut self, movement: Movement) {
		let offset = match movement.direction {
			Direction::Left => -1,
			Direction::Right => 1,
		} * movement.amount as i32;

		self.current_code = (self.current_code + offset).rem_euclid(self.max_code + 1);
	}

	pub fn current_code(self) -> i32 {
		self.current_code
	}
}
