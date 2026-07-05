use egui;

use crate::ui::{value_bar_converter::ValueBarConverter, widgets::reduced_text::FancyNumber};

pub struct FancySlider<'a> {
    value: &'a mut u32,
    sign: bool,
    label: &'a str,
    converter: &'a ValueBarConverter,
}

impl<'a> FancySlider<'a> {
    pub fn new(
        value: &'a mut u32,
        sign: bool,
        label: &'a str,
        converter: &'a ValueBarConverter,
    ) -> Self {
        Self {
            value,
            sign,
            label,
            converter,
        }
    }

    fn text(&self) -> FancyNumber {
        let actual_value = self.converter.to_value(self.value.clone());
        FancyNumber::new(actual_value, self.sign)
    }
}

impl egui::Widget for FancySlider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label(self.label);
            let fancy_number = self.text();
            ui.add(
                egui::Slider::new(self.value, 0..=120)
                    .text(fancy_number.text())
                    .show_value(false),
            );

            ui.separator();
        })
        .response
    }
}
