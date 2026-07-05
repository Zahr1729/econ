use eframe::egui;
use egui::{Color32, RichText, Widget};

pub fn reduce_text_u64(value: u64, sign: SignEnum) -> RichText {
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
    RichText::new(text).color(match sign {
        SignEnum::Positive => Color32::GREEN,
        SignEnum::Negative => Color32::RED,
        SignEnum::Neutral => Color32::YELLOW,
    })
}

pub fn reduce_text_f64(value: f64, sign: SignEnum) -> RichText {
    let text = if value > 1_000_000_000_000.0 {
        let adjusted_value = (value / 10_000_000_000.0) / 100.0;
        format!("{:.6} T", adjusted_value.to_string())
    } else if value > 1_000_000_000.0 {
        let adjusted_value = (value / 10_000_000.0) / 100.0;
        format!("{:.6} B", adjusted_value.to_string())
    } else if value > 1_000_000.0 {
        let adjusted_value = (value / 10_000.0) / 100.0;
        format!("{:.6} M", adjusted_value.to_string())
    } else {
        format!("{}", value)
    };
    RichText::new(text).color(match sign {
        SignEnum::Positive => Color32::GREEN,
        SignEnum::Negative => Color32::RED,
        SignEnum::Neutral => Color32::YELLOW,
    })
}

pub enum Number {
    U(u64),
    F(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum SignEnum {
    Positive,
    Negative,
    Neutral,
}

pub struct FancyNumber {
    value: Number,
    sign: SignEnum,
}

impl FancyNumber {
    pub fn new(value: Number, sign: SignEnum) -> Self {
        Self { value, sign }
    }

    pub fn text(&self) -> RichText {
        match self.value {
            Number::U(u) => reduce_text_u64(u, self.sign),
            Number::F(f) => reduce_text_f64(f, self.sign),
        }
    }
}

impl Widget for FancyNumber {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(egui::Label::new(self.text()).selectable(false))
    }
}
