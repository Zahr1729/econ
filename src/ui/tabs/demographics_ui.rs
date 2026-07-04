use std::sync::{Arc, Mutex};

use crate::{
    model::Economy,
    ui::{tabs::Tab, widgets::bar_chart::BarChart},
};

use egui::vec2;

pub struct DemographicsUiHandler {
    points: Vec<u64>,
    thickness: f32,
}

impl Default for DemographicsUiHandler {
    fn default() -> Self {
        Self {
            points: (0..100)
                .map(|i| (i + 100) * (i + 100) * (i + 100) / ((101 - i) * (101 - i) * (101 - i)))
                .collect(),
            thickness: 0.5,
        }
    }
}

impl Tab for DemographicsUiHandler {
    fn ui_centre(&mut self, ui: &mut egui::Ui, _economy: Arc<Mutex<Economy>>) {
        ui.add(BarChart::new(
            &mut self.points,
            &mut self.thickness,
            1000.0 * vec2(1.0, 0.35),
        ));
    }

    fn ui_left(
        &mut self,
        _ui: &mut egui::Ui,
        _economy: std::sync::Arc<std::sync::Mutex<crate::model::Economy>>,
    ) {
        ()
    }
}
