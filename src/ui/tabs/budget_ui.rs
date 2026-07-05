use std::sync::{Arc, Mutex};

use crate::{
    model::Economy,
    ui::{tabs::Tab, widgets::pie_chart::PieChart},
};

use egui::vec2;
use humanly::HumanNumber;

use crate::ui::{value_bar_converter::ValueBarConverter, widgets::fancy_slider::FancySlider};

pub struct BudgetUiHandler {
    spending_bar: u32,
    taxes_bar: u32,
    printing_bar: u32,
    radius: f32,
}

impl Default for BudgetUiHandler {
    fn default() -> Self {
        Self {
            spending_bar: 100,
            taxes_bar: 100,
            printing_bar: 100,
            radius: 150.0,
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
            true,
            "Taxes",
            &converter_taxes,
        ));
        ui.add(FancySlider::new(
            &mut self.spending_bar,
            false,
            "Spending",
            &converter_spending,
        ));
        ui.add(FancySlider::new(
            &mut self.printing_bar,
            true,
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
        ui.label(format!("Inflation: {}", HumanNumber::from(state.inflation)));
        ui.label(format!(
            "Interest: {}",
            HumanNumber::from(state.interest).concise()
        ));
        ui.label(format!(
            "Surplus: {}",
            HumanNumber::from(state.surplus() as f64).concise()
        ));
        ui.label(format!(
            "Debt: {}",
            HumanNumber::from(state.debt as f64).concise()
        ));
        ui.label(format!(
            "Money Supply: {}",
            HumanNumber::from(state.money_supply as f64).concise()
        ));
    }

    fn ui_centre(&mut self, ui: &mut egui::Ui, economy: Arc<Mutex<Economy>>) {
        let state = &mut economy.lock().unwrap().state;
        let mut v = vec![state.taxes, state.printing, state.borrowing];
        ui.add(PieChart::new(
            &mut v,
            &mut self.radius,
            1000.0 * vec2(1.0, 0.35),
        ));
    }
}
