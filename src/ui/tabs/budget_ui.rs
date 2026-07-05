use crate::{
    model::Economy,
    ui::{
        tabs::Tab,
        widgets::{
            fancy_slider::SliderHandler,
            fancy_text::{FancyNumber, Number, SignEnum},
            pie_chart::PieChart,
            stacked_bar_chart::StackedBarChart,
        },
    },
};
use eframe::egui;
use std::sync::{Arc, Mutex};

use egui::{Widget, vec2};

use crate::ui::{value_bar_converter::ValueBarConverter, widgets::fancy_slider::FancySlider};

pub struct BudgetUiHandler {
    pub taxes_bar_handler: SliderHandler,
    pub spending_bar_handler: SliderHandler,
    pub printing_bar_handler: SliderHandler,
    radius: f32,
    vertical: bool,
}

impl Default for BudgetUiHandler {
    fn default() -> Self {
        let converter_taxes = ValueBarConverter::new(0, 132_040_560_000, 0, 120);
        let converter_spending = ValueBarConverter::new(0, 181_216_060_000, 0, 120);
        let converter_printing = ValueBarConverter::new(0, 581_024_030_000, 0, 120);

        Self {
            taxes_bar_handler: SliderHandler::new(100, "Taxes".to_string(), converter_taxes),
            spending_bar_handler: SliderHandler::new(
                100,
                "Spending".to_string(),
                converter_spending,
            ),
            printing_bar_handler: SliderHandler::new(
                20,
                "Printing".to_string(),
                converter_printing,
            ),
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

        ui.add(FancySlider::new(
            &mut self.taxes_bar_handler,
            SignEnum::Positive,
        ));
        ui.add(FancySlider::new(
            &mut self.printing_bar_handler,
            SignEnum::Positive,
        ));

        ui.add(FancySlider::new(
            &mut self.spending_bar_handler,
            SignEnum::Negative,
        ));

        let button = ui.button("Progress Year");
        state.taxes = self.taxes_bar_handler.to_value();
        state.spending = self.spending_bar_handler.to_value();
        state.printing = self.printing_bar_handler.to_value();

        if button.clicked() {
            state.progress_year();
        }
        state.adjust_borrowing();
        ui.horizontal(|ui| {
            FancyNumber::new(
                "Inflation: ".to_string(),
                Number::F(state.inflation),
                SignEnum::Neutral,
            )
            .ui(ui)
        });
        ui.horizontal(|ui| {
            FancyNumber::new(
                "Interest: ".to_string(),
                Number::F(state.interest),
                SignEnum::Neutral,
            )
            .ui(ui)
        });
        ui.horizontal(|ui| {
            FancyNumber::new(
                "Money Supply: ".to_string(),
                Number::U(state.money_supply),
                SignEnum::Neutral,
            )
            .ui(ui)
        });
        ui.separator();
        ui.horizontal(|ui| {
            FancyNumber::new(
                "Surplus: ".to_string(),
                Number::U(state.surplus()),
                SignEnum::Positive,
            )
            .ui(ui)
        });
        ui.separator();
        ui.horizontal(|ui| {
            FancyNumber::new(
                "Interest: ".to_string(),
                Number::U(state.interest_payments),
                SignEnum::Negative,
            )
            .ui(ui)
        });
        ui.horizontal(|ui| {
            FancyNumber::new(
                "Debt: ".to_string(),
                Number::U(state.debt),
                SignEnum::Negative,
            )
            .ui(ui)
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
