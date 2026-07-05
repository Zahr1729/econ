use eframe::egui;
use egui::{Color32, Pos2, Rect, Response, Stroke, Ui, Vec2, Widget, emath, vec2};
use std::{
    f32::consts::PI,
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct PieChart<'a> {
    data: &'a mut Vec<u64>,

    radius: &'a mut f32,
    size: Vec2,
}

impl<'a> PieChart<'a> {
    pub fn new(data: &'a mut Vec<u64>, radius: &'a mut f32, size: Vec2) -> Self {
        PieChart { data, radius, size }
    }
}

impl Widget for PieChart<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::canvas(ui.style())
            .show(ui, |ui| {
                let mut shapes = vec![];

                let (_id, rect) = ui.allocate_space(self.size);

                let total_value = self.data.iter().sum::<u64>().max(1);

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_x_y_ranges(0.0..=1.0 as f32, 0.0..=1.0),
                    rect,
                );

                let midpoint = to_screen * Pos2::new(0.5, 0.5);
                // let radius = to_screen.scale().min_elem();

                let mut runnning_total = 0;

                for i in 0..self.data.len() {
                    let colour_hasher = &mut DefaultHasher::new();
                    i.hash(colour_hasher);
                    let r = colour_hasher.finish();
                    i.hash(colour_hasher);
                    let g = colour_hasher.finish();
                    i.hash(colour_hasher);
                    let b = colour_hasher.finish();
                    let colour = Color32::from_rgb(
                        0 + ((r) / (u64::MAX / 255)) as u8,
                        128 + ((g) / (u64::MAX / 127)) as u8,
                        200 - ((b) / (u64::MAX / 55)) as u8,
                    );

                    let initial_angle = (runnning_total as f32 / total_value as f32) * 2.0 * PI;

                    let angle = (self.data[i] as f32 / total_value as f32) * 2.0 * PI;

                    let num_points = (angle * 100.0 / (2.0 * PI)) as usize;

                    let mut points: Vec<_> = vec![vec2(*self.radius, 0.0); num_points + 1]
                        .iter()
                        .enumerate()
                        .map(|(i, v)| {
                            let this_angle = initial_angle + angle * (i as f32 / num_points as f32);
                            midpoint
                                + vec2(
                                    v.x * this_angle.cos() + v.y * this_angle.sin(),
                                    v.y * this_angle.cos() - v.x * this_angle.sin(),
                                )
                        })
                        .collect();

                    points.push(midpoint);

                    let stroke = Stroke {
                        width: 3.0,
                        color: Color32::DARK_GRAY,
                    };

                    let sector = egui::Shape::convex_polygon(points, colour, stroke);

                    shapes.push(sector);

                    runnning_total += self.data[i];
                }
                ui.painter().extend(shapes);

                ui.add(egui::Slider::new(self.radius, 0.0..=200.0).text("Radius"))
            })
            .response
    }
}
