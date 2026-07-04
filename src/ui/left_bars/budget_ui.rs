use crate::ui::UiHandler;

use humanly::HumanNumber;

use crate::ui::{value_bar_converter::ValueBarConverter, widgets::fancy_slider::FancySlider};

impl UiHandler {
    pub fn budget_ui(&mut self, ui: &mut egui::Ui) {
        let state = &mut self.economy.lock().unwrap().state;
        ui.heading("Budget");

        ui.add(FancySlider::new(
            &mut self.taxes_bar,
            "Taxes",
            &ValueBarConverter::new(0, 132_040_560_000, 0, 120),
        ));
        ui.add(FancySlider::new(
            &mut self.spending_bar,
            "Spending",
            &ValueBarConverter::new(0, 181_216_060_000, 0, 120),
        ));
        ui.add(FancySlider::new(
            &mut self.printing_bar,
            "Printing",
            &ValueBarConverter::new(0, 51_024_030_000, 0, 120),
        ));

        let button = ui.button("Progress Year");
        state.taxes = self.taxes_bar as u64 * 200_000_000_000 / 100;
        state.spending = self.spending_bar as u64 * 200_000_000_000 / 100;
        state.printing = self.printing_bar as u64 * 200_000_000_000 / 100;

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
}
