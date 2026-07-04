use egui::{
    Color32, Pos2, Rect, Response, Stroke, Ui, Vec2, Widget, emath,
    epaint::{self, CircleShape},
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

                for i in 0..self.data.len() {
                    let colour = Color32::from_rgb(
                        0,
                        128 + ((self.data[i] * 127) / total_value) as u8,
                        200 - ((self.data[i] * 55) / total_value) as u8,
                    );
                    let circle = CircleShape {
                        center: midpoint,
                        radius: *self.radius,
                        fill: colour,
                        stroke: Stroke {
                            width: 1.0,
                            color: Color32::GRAY,
                        },
                    };
                    shapes.push(epaint::Shape::Circle(circle));
                }
                ui.painter().extend(shapes);

                ui.add(egui::Slider::new(self.radius, 0.0..=200.0).text("Radius"))
            })
            .response
    }
}
