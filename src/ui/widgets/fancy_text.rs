use std::iter::Sum;

use eframe::egui::{self, WidgetText};
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

pub fn reduce_text(number: &FancyNumber) -> RichText {
    match number.value {
        Number::U(v) => reduce_text_u64(v, number.sign),
        Number::F(v) => reduce_text_f64(v, number.sign),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    U(u64),
    F(f64),
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SignEnum {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FancyNumber {
    pub name: String,
    pub value: Number,
    pub sign: SignEnum,
    pub show_name: bool,
}

impl FancyNumber {
    pub fn new(name: String, value: Number, sign: SignEnum) -> Self {
        Self {
            name,
            value,
            sign,
            show_name: true,
        }
    }

    pub fn numeric_text(&self) -> RichText {
        match self.value {
            Number::U(u) => reduce_text_u64(u, self.sign),
            Number::F(f) => reduce_text_f64(f, self.sign),
        }
    }

    pub fn show_name(&mut self, show_name: bool) {
        self.show_name = show_name;
    }
}

impl Sum for &FancyNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let value = iter.map(|fancy_number| fancy_number.value).sum();
        let name = "".to_string();
        &FancyNumber::new(name, value, SignEnum::Neutral)
    }
}

impl Into<WidgetText> for &FancyNumber {
    fn into(self) -> WidgetText {
        self.numeric_text().into()
    }
}

impl Widget for &FancyNumber {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if self.show_name {
                ui.add(egui::Label::new(&self.name).selectable(false));
            }
            ui.add(egui::Label::new(self.numeric_text()).selectable(false))
        })
        .response
    }
}
