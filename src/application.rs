use std::f64::consts::TAU;

use crate::structs::State;
use eframe::{egui, epaint::Color32};

pub struct Application {
	pub state: State,
	pub step: usize,
	pub version: String,
}

impl Application {
	pub fn new(cc: &eframe::CreationContext<'_>, version: String) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		Self {
			state: State::default(),
			step: 0,
			version,
		}
	}
	
	pub fn step(&mut self) {
		for i in 0..self.state.positions.len() {
			let position = self.state.positions[i];
			if self.state.running && position.length() >= self.state.speed * self.state.timestep {
				let previous_snail_position = self.state.positions[if i > 0 { i - 1 } else { self.state.positions.len() - 1 }];
				let direction = (previous_snail_position - position).normalized();
				let speed = direction * self.state.speed;
				let new_position = position + speed * self.state.timestep;
				self.state.previous_positions[i].push(new_position);
			}
		}
		for i in 0..self.state.positions.len() {
			if self.state.previous_positions[i].is_empty() {
				continue;
			}
			self.state.positions[i] = self.state.previous_positions[i][self.state.previous_positions[i].len() - 1];
		}
	}

	pub fn render(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {
				let run_pause_button = ui.button(if self.state.running { "Pause" } else { "Run" });
				if run_pause_button.clicked() {
					self.state.running = !self.state.running;
				}
				ui.add_space(10.0);

				let snails_count = self.state.snails_count;
				ui.add_enabled(!self.state.running, egui::DragValue::new(&mut self.state.snails_count).speed(0.1));
				ui.label("Snails count");
				ui.add_space(10.0);
				
				ui.add(egui::DragValue::new(&mut self.state.speed).speed(0.01));
				ui.label("Snails speed");
				ui.add_space(10.0);
				
				let radius = self.state.radius;
				ui.add_enabled(!self.state.running, egui::DragValue::new(&mut self.state.radius).speed(0.01));
				ui.label("Circle radius");
				ui.add_space(10.0);
				
				ui.add(egui::DragValue::new(&mut self.state.timestep).speed(0.001));
				ui.label("Timestep");
				if self.state.timestep < 0.0 {
					self.state.timestep = 0.0;
				}
				ui.add_space(10.0);

				ui.add(egui::DragValue::new(&mut self.state.steps_per_frame).speed(0.01));
				ui.label("Simulation steps per frame");
				ui.add_space(10.0);

				let reset_button = ui.button("Reset the animation");

				if self.state.snails_count != snails_count || self.state.radius != radius || reset_button.clicked() {
					self.state.reinitialise();
				}
			});

			let plot = egui::plot::Plot::new("Data").data_aspect(1.0);
			// Circle
			let n = 512;
			let circle_points: egui::plot::PlotPoints = (0..=n)
				.map(|i| {
					let t = eframe::emath::remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
					let r = self.state.radius;
					let (x, y) = t.sin_cos();
					[x * r, y * r]
				})
				.collect();
			let circle_line = egui::plot::Line::new(circle_points).color(Color32::GRAY);
			
			// Snails
			let mut all_points = vec![];
			let mut all_lines = vec![];
			for i in 0..self.state.positions.len() {
				let colour = crate::COLOURS[i % crate::COLOURS.len()];
				
				let graph_points_raw = self.state.previous_positions[i].iter().map(|point| [point.x as f64, point.y as f64]).collect::<Vec<[f64; 2]>>();
				let graph_points = egui::plot::Points::new(graph_points_raw.clone())
					.color(colour)
					.highlight(true);
				all_points.push(graph_points);
				let snail_line = {
					let line_points: egui::plot::PlotPoints = graph_points_raw.into_iter().collect();
					egui::plot::Line::new(line_points).color(colour)
				};
				all_lines.push(snail_line);
			}

			// The n-agon
			let mut n_agon_lines = vec![];
			for i in 0..self.state.previous_positions.len() {
				let position = self.state.previous_positions[i][0];
				let previous_position = self.state.previous_positions[if i > 0 { i - 1 } else { self.state.previous_positions.len() - 1 }][0];
				let line = egui::plot::Line::new(vec![[position.x as f64, position.y as f64], [previous_position.x as f64, previous_position.y as f64]]).color(Color32::from_rgba_unmultiplied(160, 160, 160, 160));
				n_agon_lines.push(line);
			}

			// Render the plot
			plot.show(ui, |plot_ui| {
				for n_agon_line in n_agon_lines {
					plot_ui.line(n_agon_line);
				}
				plot_ui.line(circle_line);
				for points in all_points {
					plot_ui.points(points);
				}
				for snail_line in all_lines {
					plot_ui.line(snail_line);
				}
			});
		});
	}
}
