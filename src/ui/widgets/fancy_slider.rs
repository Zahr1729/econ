use egui::{self, Widget};

use crate::ui::{value_bar_converter::ValueBarConverter, widgets::reduced_text::reduce_text};

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
        reduce_text(actual_value)
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

            ui.separator();
        })
        .response
    }
}
