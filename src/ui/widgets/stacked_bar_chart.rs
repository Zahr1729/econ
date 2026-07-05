use eframe::egui;
use egui_plot;

use crate::model::state::State;

pub struct StackedBarChart<'a> {
    state: &'a State,
    vertical: &'a mut bool,
}

impl<'a> StackedBarChart<'a> {
    pub fn new(state: &'a State, vertical: &'a mut bool) -> Self {
        Self { state, vertical }
    }
}

impl StackedBarChart<'_> {
    pub fn show_controls(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("Orientation:");
            ui.selectable_value(self.vertical, true, "Vertical");
            ui.selectable_value(self.vertical, false, "Horizontal");
        })
        .response
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) -> egui::Response {
        let mut taxes = egui_plot::BarChart::new(
            "taxes",
            vec![egui_plot::Bar::new(0.5, self.state.taxes as f64)],
        )
        .width(0.7)
        .name("Taxes");

        let mut printing = egui_plot::BarChart::new(
            "printing",
            vec![egui_plot::Bar::new(0.5, self.state.printing as f64)],
        )
        .width(0.7)
        .name("Printing");

        let mut borrowing = egui_plot::BarChart::new(
            "borrowing",
            vec![egui_plot::Bar::new(0.5, self.state.borrowing as f64)],
        )
        .width(0.7)
        .name("Borrowing");

        // Expenses
        let mut spending = egui_plot::BarChart::new(
            "spending",
            vec![egui_plot::Bar::new(1.5, self.state.spending as f64)],
        )
        .width(0.7)
        .name("Spending");

        let mut interest_payments = egui_plot::BarChart::new(
            "interest_payments",
            vec![egui_plot::Bar::new(
                1.5,
                self.state.interest_payments as f64,
            )],
        )
        .width(0.7)
        .name("Interest Payments");

        if !*self.vertical {
            taxes = taxes.horizontal();
            printing = printing.horizontal();
            borrowing = borrowing.horizontal();
            spending = spending.horizontal();
            interest_payments = interest_payments.horizontal();
        }

        let plot_response = egui_plot::Plot::new("Stacked Bar Chart Demo")
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(taxes);
                plot_ui.bar_chart(printing);
                plot_ui.bar_chart(borrowing);
                plot_ui.bar_chart(spending);
                plot_ui.bar_chart(interest_payments);
            });
        plot_response.response
    }
}

impl egui::Widget for StackedBarChart<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        self.show_controls(ui);
        self.show_plot(ui)
    }
}
