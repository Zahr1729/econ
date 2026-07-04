use egui::{self, Widget};

use crate::ui::value_bar_converter::ValueBarConverter;

pub struct FancySlider<'a> {
    value: &'a mut u32,
    label: &'a str,
    converter: &'a ValueBarConverter,
}

impl<'a> FancySlider<'a> {
    pub fn new(value: &'a mut u32, label: &'a str, converter: &'a ValueBarConverter) -> Self {
        Self {
            value,
            label,
            converter,
        }
    }

    fn text(&self) -> String {
        let actual_value = self.converter.to_value(self.value.clone());
        let text = if actual_value > 1_000_000_000_000 {
            let adjusted_value = (actual_value / 10_000_000_000) as f64 / 100.0;
            format!("{:.6} T", adjusted_value.to_string())
        } else if actual_value > 1_000_000_000 {
            let adjusted_value = (actual_value / 10_000_000) as f64 / 100.0;
            format!("{:.6} B", adjusted_value.to_string())
        } else if actual_value > 1_000_000 {
            let adjusted_value = (actual_value / 10_000) as f64 / 100.0;
            format!("{:.6} M", adjusted_value.to_string())
        } else {
            format!("{}", actual_value)
        };
        text
    }
}

impl Widget for FancySlider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label(self.label);

            let text = self.text();
            ui.add(
                egui::Slider::new(self.value, 0..=120)
                    .text(format!("{}", text))
                    .show_value(false),
            );
        })
        .response
    }
}
