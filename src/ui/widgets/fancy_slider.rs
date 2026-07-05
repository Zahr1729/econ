use eframe::egui;

use crate::ui::{
    value_bar_converter::ValueBarConverter,
    widgets::reduced_text::{FancyNumber, Number, SignEnum},
};

pub struct SliderHandler {
    pub value: u32,
    pub label: String,
    pub converter: ValueBarConverter,
}

impl SliderHandler {
    pub fn new(value: u32, label: String, converter: ValueBarConverter) -> Self {
        Self {
            value,
            label,
            converter,
        }
    }

    pub fn to_value(&self) -> u64 {
        self.converter.to_value(self.value)
    }
}

pub struct FancySlider<'a> {
    sign: SignEnum,
    handler: &'a mut SliderHandler,
}

impl<'a> FancySlider<'a> {
    pub fn new(handler: &'a mut SliderHandler, sign: SignEnum) -> Self {
        Self { sign, handler }
    }

    fn text(&self) -> FancyNumber {
        let actual_value = self.handler.converter.to_value(self.handler.value.clone());
        FancyNumber::new(Number::U(actual_value), self.sign)
    }
}

impl egui::Widget for FancySlider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label(self.handler.label.to_string());
            let fancy_number = self.text();
            ui.add(
                egui::Slider::new(&mut self.handler.value, 0..=120)
                    .text(fancy_number.text())
                    .show_value(false),
            );

            ui.separator();
        })
        .response
    }
}
