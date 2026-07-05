pub mod tabs;
pub mod value_bar_converter;
pub mod widgets;

use std::sync::{Arc, Mutex};

use egui::{self, Widget};

use crate::{
    model::Economy,
    ui::{
        tabs::{Tab, budget_ui::BudgetUiHandler, demographics_ui::DemographicsUiHandler},
        widgets::reduced_text::reduce_text,
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
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
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
                    egui::Label::new(reduce_text(state.deficit(), false))
                        .selectable(false)
                        .ui(ui)
                } else {
                    egui::Label::new(reduce_text(state.surplus(), true))
                        .selectable(false)
                        .ui(ui)
                };

                // Critical data.
                let tooltip = egui::Tooltip::for_enabled(&response);
                tooltip.show(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("Revenue: ")));
                        ui.add(egui::Label::new(reduce_text(state.revenue(), true)));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("\tTaxes: ")));
                        ui.add(egui::Label::new(reduce_text(state.taxes, true)));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("\tPrinting: ")));
                        ui.add(egui::Label::new(reduce_text(state.printing, true)));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("Expenditure: ")));
                        ui.add(egui::Label::new(reduce_text(state.expenses(), false)));
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new("\tSpending: ")));
                        ui.add(egui::Label::new(reduce_text(state.spending, false)));
                    });
                });
            })
        });
        egui::SidePanel::left("Left Panel").show(ctx, |ui| match self.tab {
            TabEnum::Budget => {
                self.budget_ui_handler.ui_left(ui, self.economy.clone());
            }
            TabEnum::Demographic => {
                ();
            }
            _ => (),
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.tab {
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
