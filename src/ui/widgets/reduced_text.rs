use egui::{Color32, RichText, Widget};

pub fn reduce_text(value: u64, sign: bool) -> RichText {
    let text = if value > 1_000_000_000_000 {
        let adjusted_value = (value / 10_000_000_000) as f64 / 100.0;
        format!("{:.6} T", adjusted_value.to_string())
    } else if value > 1_000_000_000 {
        let adjusted_value = (value / 10_000_000) as f64 / 100.0;
        format!("{:.6} B", adjusted_value.to_string())
    } else if value > 1_000_000 {
        let adjusted_value = (value / 10_000) as f64 / 100.0;
        format!("{:.6} M", adjusted_value.to_string())
    } else {
        format!("{}", value)
    };
    RichText::new(text).color(if sign { Color32::GREEN } else { Color32::RED })
}

pub struct FancyNumber {
    value: u64,
    sign: bool,
}

impl FancyNumber {
    pub fn new(value: u64, sign: bool) -> Self {
        Self { value, sign }
    }

    pub fn text(&self) -> RichText {
        reduce_text(self.value, self.sign)
    }
}

impl Widget for FancyNumber {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(egui::Label::new(self.text()).selectable(false))
    }
}
