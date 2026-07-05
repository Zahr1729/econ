use crate::{
    model::Economy,
    ui::{
        tabs::Tab,
        widgets::{
            pie_chart::PieChart,
            reduced_text::{FancyNumber, Number, SignEnum},
            stacked_bar_chart::StackedBarChart,
        },
    },
};
use eframe::egui;
use std::sync::{Arc, Mutex};

use egui::{Widget, vec2};

use crate::ui::{value_bar_converter::ValueBarConverter, widgets::fancy_slider::FancySlider};

pub struct BudgetUiHandler {
    spending_bar: u32,
    taxes_bar: u32,
    printing_bar: u32,
    radius: f32,
    vertical: bool,
}

impl Default for BudgetUiHandler {
    fn default() -> Self {
        Self {
            spending_bar: 100,
            taxes_bar: 100,
            printing_bar: 100,
            radius: 150.0,
            vertical: true,
        }
    }
}

impl Tab for BudgetUiHandler {
    fn ui_left(&mut self, ui: &mut egui::Ui, economy: Arc<Mutex<Economy>>) {
        let state = &mut economy.lock().unwrap().state;
        ui.heading("Budget");
        ui.separator();

        let converter_taxes = ValueBarConverter::new(0, 132_040_560_000, 0, 120);
        let converter_spending = ValueBarConverter::new(0, 181_216_060_000, 0, 120);
        let converter_printing = ValueBarConverter::new(0, 51_024_030_000, 0, 120);

        ui.add(FancySlider::new(
            &mut self.taxes_bar,
            SignEnum::Positive,
            "Taxes",
            &converter_taxes,
        ));
        ui.add(FancySlider::new(
            &mut self.spending_bar,
            SignEnum::Negative,
            "Spending",
            &converter_spending,
        ));
        ui.add(FancySlider::new(
            &mut self.printing_bar,
            SignEnum::Positive,
            "Printing",
            &converter_printing,
        ));

        let button = ui.button("Progress Year");
        state.taxes = converter_taxes.to_value(self.taxes_bar);
        state.spending = converter_spending.to_value(self.spending_bar);
        state.printing = converter_printing.to_value(self.printing_bar);

        if button.clicked() {
            state.progress_year();
        }
        state.adjust_borrowing();
        ui.horizontal(|ui| {
            ui.label("Inflation: ");
            FancyNumber::new(Number::F(state.inflation), SignEnum::Neutral).ui(ui)
        });
        ui.horizontal(|ui| {
            ui.label("Interest: ");
            FancyNumber::new(Number::F(state.interest), SignEnum::Neutral).ui(ui)
        });
        ui.horizontal(|ui| {
            ui.label("Money Supply: ");
            FancyNumber::new(Number::U(state.money_supply), SignEnum::Neutral).ui(ui)
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Surplus: ");
            FancyNumber::new(Number::U(state.surplus()), SignEnum::Positive).ui(ui)
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Interest: ");
            FancyNumber::new(Number::U(state.deficit()), SignEnum::Negative).ui(ui)
        });
        ui.horizontal(|ui| {
            ui.label("Debt: ");
            FancyNumber::new(Number::U(state.debt), SignEnum::Negative).ui(ui)
        });
    }

    fn ui_centre(&mut self, ui: &mut egui::Ui, economy: Arc<Mutex<Economy>>) {
        let state = &mut economy.lock().unwrap().state;
        let mut v = vec![state.taxes, state.printing, state.borrowing];
        ui.add(PieChart::new(
            &mut v,
            &mut self.radius,
            1000.0 * vec2(1.0, 0.35),
        ));

        ui.add(StackedBarChart::new(state, &mut self.vertical));
    }
}
