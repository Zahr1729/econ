use eframe::egui;
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2, Widget, emath, epaint};

pub struct BarChart<'a> {
    points: &'a mut Vec<u64>,
    thickness: &'a mut f32,
    size: Vec2,
}

impl<'a> BarChart<'a> {
    pub fn new(points: &'a mut Vec<u64>, thickness: &'a mut f32, size: Vec2) -> Self {
        BarChart {
            points,
            thickness,
            size,
        }
    }
}

impl Widget for BarChart<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::canvas(ui.style())
            .show(ui, |ui| {
                let mut shapes = vec![];

                let (_id, rect) = ui.allocate_space(self.size);

                let max_value = self.points.iter().max().unwrap();

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_x_y_ranges(-1.0..=self.points.len() as f32, 0.0..=*max_value as f32),
                    rect,
                );

                for i in 0..self.points.len() {
                    let colour = Color32::from_rgb(
                        0,
                        128 + ((self.points[i] * 127) / max_value) as u8,
                        200 - ((self.points[i] * 55) / max_value) as u8,
                    );
                    let min = to_screen
                        * Pos2 {
                            x: i as f32,
                            y: (*max_value - self.points[i]) as f32,
                        }
                        + to_screen.scale()
                            * Vec2 {
                                x: -self.thickness.clone() / 2.0,
                                y: 0.0,
                            };
                    let max = to_screen
                        * Pos2 {
                            x: i as f32,
                            y: *max_value as f32,
                        }
                        + to_screen.scale()
                            * Vec2 {
                                x: self.thickness.clone() / 2.0,
                                y: 0.0,
                            };
                    let rect = Rect { min, max };
                    shapes.push(epaint::Shape::rect_filled(rect, 0.0, colour));
                }
                ui.painter().extend(shapes);

                ui.add(egui::Slider::new(self.thickness, 0.0..=1.0).text("Thickness"))
            })
            .response
    }
}
