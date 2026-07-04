use std::sync::{Arc, Mutex};

use crate::model::Economy;

pub mod budget_ui;
pub mod demographics_ui;

pub trait Tab {
    fn ui_left(&mut self, ui: &mut egui::Ui, economy: Arc<Mutex<Economy>>);
    fn ui_centre(&mut self, ui: &mut egui::Ui, economy: Arc<Mutex<Economy>>);
}
