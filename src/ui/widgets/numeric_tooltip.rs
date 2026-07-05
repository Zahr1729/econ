use crate::ui::widgets::fancy_text::{FancyNumber, SignEnum, reduce_text, reduce_text_u64};
use eframe::egui::{self, Response, Ui};

pub struct NumericTooltip {
    pub value: FancyNumber,
    pub constituents: Vec<FancyNumber>,
}

impl NumericTooltip {
    pub fn new(value: FancyNumber, constituents: Vec<FancyNumber>) -> Self {
        Self {
            value,
            constituents,
        }
    }

    fn get_signed_data(&self, sign: SignEnum) -> Vec<&FancyNumber> {
        let signed_data: Vec<_> = self
            .constituents
            .iter()
            .filter_map(|num| if num.sign == sign { Some(num) } else { None })
            .collect();

        signed_data
    }

    fn draw_signed_data(&self, ui: &mut egui::Ui, sign: SignEnum) {
        let signed_data = self.get_signed_data(sign);

        let total = signed_data.iter().sum();

        for number in signed_data {
            ui.horizontal(|ui| ui.add(number));
        }
    }
}

impl egui::Widget for NumericTooltip {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let label = reduce_text(&self.value);
        let response = ui.label(label);

        let tooltip = egui::Tooltip::for_enabled(&response);

        tooltip.show(|ui| {
            // Get Positive data

            // Critical data.
        });

        response
    }
}
