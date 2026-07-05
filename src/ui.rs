pub mod tabs;
pub mod utils;
pub mod value_bar_converter;
pub mod widgets;

use eframe::egui::{self, Ui};
use egui::Widget;
use std::sync::{Arc, Mutex};

use crate::{
    model::Economy,
    ui::{
        tabs::{Tab, budget_ui::BudgetUiHandler, demographics_ui::DemographicsUiHandler},
        widgets::{
            fancy_text::{FancyNumber, Number, SignEnum, reduce_text_u64},
            numeric_tooltip::NumericTooltip,
        },
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum TabEnum {
    Political,
    Demographic,
    Budget,
}

pub struct UiHandler {
    economy: Arc<Mutex<Economy>>,
    tab: TabEnum,
    budget_ui_handler: BudgetUiHandler,
    demographics_ui_handler: DemographicsUiHandler,
}

impl UiHandler {
    pub fn new(economy: Arc<Mutex<Economy>>) -> Self {
        Self {
            economy,
            tab: TabEnum::Budget,
            budget_ui_handler: BudgetUiHandler::default(),
            demographics_ui_handler: DemographicsUiHandler::default(),
        }
    }
}

impl UiHandler {
    pub fn get_budget_tooltip(&self, ui: &mut Ui) {
        let state = &self.economy.lock().unwrap().state;

        let surplus_deficit = if state.deficit() >= 0 {
            FancyNumber::new(
                "Deficit".to_string(),
                Number::U(state.deficit()),
                SignEnum::Negative,
            )
        } else {
            FancyNumber::new(
                "Surplus".to_string(),
                Number::U(state.surplus()),
                SignEnum::Positive,
            )
        };

        let vec = vec![
            FancyNumber::new(
                "Taxes".to_string(),
                Number::U(state.taxes),
                SignEnum::Positive,
            ),
            FancyNumber::new(
                "Printing".to_string(),
                Number::U(state.printing),
                SignEnum::Positive,
            ),
            FancyNumber::new(
                "Spending".to_string(),
                Number::U(state.spending),
                SignEnum::Negative,
            ),
            FancyNumber::new(
                "Interest".to_string(),
                Number::U(state.interest_payments),
                SignEnum::Negative,
            ),
        ];

        ui.add(NumericTooltip::new(surplus_deficit, vec));
    }

    pub fn update(&mut self, ui: &mut Ui) {
        egui::Panel::top("Top Panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                let tab = &mut self.tab;
                // Tab choices
                ui.horizontal(|ui| {
                    ui.selectable_value(tab, TabEnum::Political, "Politics");
                    ui.selectable_value(tab, TabEnum::Budget, "Budget");
                    ui.selectable_value(tab, TabEnum::Demographic, "Demography");
                });
                ui.separator();

                // Budget topbar
                self.get_budget_tooltip(ui);
            });
        });

        egui::Panel::left("Left Panel").show(ui, |ui| match self.tab {
            TabEnum::Budget => {
                self.budget_ui_handler.ui_left(ui, self.economy.clone());
            }
            TabEnum::Demographic => {
                ();
            }
            _ => (),
        });

        egui::CentralPanel::default().show(ui, |ui| match self.tab {
            TabEnum::Budget => {
                self.budget_ui_handler.ui_centre(ui, self.economy.clone());
            }
            TabEnum::Demographic => {
                self.demographics_ui_handler
                    .ui_centre(ui, self.economy.clone());
            }
            _ => (),
        });
    }
}
