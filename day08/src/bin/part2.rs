use std::io;

use color_eyre::eyre;

use day08::{Circuit, Coord, DistancePair};

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	let mut coords = Vec::<Coord>::with_capacity(1000);
	let mut nearest_pairs = Vec::<DistancePair>::new();
	let mut circuits = Vec::<Circuit>::new();

	for line in io::stdin().lines() {
		coords.push(line?.parse()?);
	}

	for (first, &first_coord) in coords.iter().enumerate().take(coords.len() - 1) {
		for (second, &second_coord) in coords.iter().enumerate().skip(first + 1) {
			let square_distance = first_coord.square_distance(second_coord);
			nearest_pairs.push(DistancePair {
				square_distance,
				first,
				second,
			});
		}
	}
	nearest_pairs.sort_unstable();

	for pair in nearest_pairs {
		let first = circuits
			.iter()
			.enumerate()
			.find_map(|(i, c)| c.contains(pair.first).then_some(i));
		let second = circuits
			.iter()
			.enumerate()
			.find_map(|(i, c)| c.contains(pair.second).then_some(i));

		if first.is_none() && second.is_none() {
			let mut new_circuit = Circuit::default();
			new_circuit.insert(pair.first);
			new_circuit.insert(pair.second);
			circuits.push(new_circuit);
		} else if let (Some(first), Some(second)) = (first, second)
			&& first != second
		{
			let other = circuits.swap_remove(first.max(second));
			circuits[first.min(second)].merge(other);
		} else if let Some(first) = first
			&& second.is_none()
		{
			circuits[first].insert(pair.second);
		} else if let Some(second) = second
			&& first.is_none()
		{
			circuits[second].insert(pair.first);
		}

		if circuits[0].len() == coords.len() {
			let x_first = coords[pair.first].x;
			let x_second = coords[pair.second].x;
			println!("{}", x_first * x_second);
			break;
		}
	}

	Ok(())
}
