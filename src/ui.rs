pub mod tabs;
pub mod value_bar_converter;
pub mod widgets;

use eframe::egui::{self, Ui};
use egui::Widget;
use std::sync::{Arc, Mutex};

use crate::{
    model::Economy,
    ui::{
        tabs::{Tab, budget_ui::BudgetUiHandler, demographics_ui::DemographicsUiHandler},
        widgets::reduced_text::{SignEnum, reduce_text_u64},
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

                let state = &self.economy.lock().unwrap().state;
                let response = if state.deficit() != 0 {
                    egui::Label::new(reduce_text_u64(state.deficit(), SignEnum::Negative))
                        .selectable(false)
                        .ui(ui)
                } else {
                    egui::Label::new(reduce_text_u64(state.surplus(), SignEnum::Positive))
                        .selectable(false)
                        .ui(ui)
                };

                // Critical data.
                let tooltip = egui::Tooltip::for_enabled(&response);
                tooltip.show(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("Revenue: ")));
                        ui.add(egui::Label::new(reduce_text_u64(
                            state.revenue(),
                            SignEnum::Positive,
                        )));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("\tTaxes: ")));
                        ui.add(egui::Label::new(reduce_text_u64(
                            state.taxes,
                            SignEnum::Positive,
                        )));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("\tPrinting: ")));
                        ui.add(egui::Label::new(reduce_text_u64(
                            state.printing,
                            SignEnum::Positive,
                        )));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("Expenditure: ")));
                        ui.add(egui::Label::new(reduce_text_u64(
                            state.expenses(),
                            SignEnum::Negative,
                        )));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("\tSpending: ")));
                        ui.add(egui::Label::new(reduce_text_u64(
                            state.spending,
                            SignEnum::Negative,
                        )));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new(
                            "\tInterest Payments: ",
                        )));
                        ui.add(egui::Label::new(reduce_text_u64(
                            state.interest_payments,
                            SignEnum::Negative,
                        )));
                    });
                });
            })
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
