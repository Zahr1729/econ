pub mod left_bars;
pub mod value_bar_converter;
pub mod widgets;

use std::sync::{Arc, Mutex};

use egui;
use egui::vec2;

use crate::{model::Economy, ui::widgets::bar_chart::BarChart};

#[derive(Debug, PartialEq, Eq)]
pub enum TabEnum {
    Political,
    Demographic,
    Budget,
}

pub struct UiHandler {
    economy: Arc<Mutex<Economy>>,
    tab: TabEnum,
    spending_bar: u32,
    taxes_bar: u32,
    printing_bar: u32,
    points: Vec<u64>,
}

impl UiHandler {
    pub fn new(economy: Arc<Mutex<Economy>>) -> Self {
        Self {
            economy,
            tab: TabEnum::Budget,
            spending_bar: 100,
            taxes_bar: 100,
            printing_bar: 100,
            points: (0..100)
                .map(|i| (i + 100) * (i + 100) * (i + 100) / ((101 - i) * (101 - i) * (101 - i)))
                .collect(),
        }
    }
}

impl UiHandler {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            let tab = &mut self.tab;
            ui.horizontal(|ui| {
                ui.selectable_value(tab, TabEnum::Political, "Politics");
                ui.selectable_value(tab, TabEnum::Budget, "Budget");
                ui.selectable_value(tab, TabEnum::Demographic, "Demography");
            });
        });
        egui::SidePanel::left("Left Panel").show(ctx, |ui| match self.tab {
            TabEnum::Budget => {
                self.budget_ui(ui);
            }
            _ => (),
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(BarChart::new(
                &mut self.points,
                1000.0 * vec2(1.0, 0.35),
                0.5,
            ));
        });
    }
}
