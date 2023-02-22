use std::f32::consts::PI;

use eframe::epaint::Vec2;

#[derive(Debug)]
pub struct State {
	pub snails_count: usize,
	pub radius: f64,
	pub speed: f32,
	pub timestep: f32,

	pub positions: Vec<Vec2>,
	pub previous_positions: Vec<Vec<Vec2>>
}

impl State {
	fn get_initial_positions(snails_count: usize, radius: f64) -> Vec<Vec2> {
		let mut positions = vec![];
		for i in 0..snails_count {
			let angle = 2.0 * PI / (snails_count as f32) * (i as f32);
			let (x, y) = angle.sin_cos();
			positions.push(Vec2::new(x, y) * radius as f32)
		}
		positions
	}
}

impl Default for State {
	fn default() -> Self {
		let snails_count = 5;
		let radius = 10.0;
		let positions = Self::get_initial_positions(snails_count, radius);

		Self {
			snails_count,
			radius,
			speed: 1.0,
			timestep: 0.01,

			positions,
			previous_positions: vec![vec![]; snails_count]
		}
	}
}