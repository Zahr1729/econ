use egui;
use egui::vec2;
use humanly::HumanNumber;

use crate::{
    model::Economy,
    ui::{
        value_bar_converter::ValueBarConverter,
        widgets::{bar_chart::BarChart, fancy_slider::FancySlider},
    },
};

pub mod model;
pub mod ui;

struct MyApp {
    economy: Economy,
    spending_bar: u32,
    taxes_bar: u32,
    printing_bar: u32,
    points: Vec<u64>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            economy: Economy {
                inflation: 0.01,
                interest: 0.02,
                debt: 3_000_000_000_000,
                taxes: 80_000_000_000,
                borrowing: 20_000_000_000,
                spending: 117_000_000_000,
                printing: 17_000_000_000,
                money_supply: 40_000_000_000_000,
            },
            spending_bar: 100,
            taxes_bar: 100,
            printing_bar: 100,
            points: (0..100).map(|i| (i + 30) * (i + 10)).collect(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Right Panel").show(ctx, |ui| {
            let economy = &mut self.economy;
            ui.heading("Economy");

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
            economy.taxes = self.taxes_bar as u64 * 200_000_000_000 / 100;
            economy.spending = self.spending_bar as u64 * 200_000_000_000 / 100;
            economy.printing = self.printing_bar as u64 * 200_000_000_000 / 100;

            if button.clicked() {
                economy.progress_year();
            }
            economy.adjust_borrowing();
            ui.label(format!(
                "Inflation: {}",
                HumanNumber::from(economy.inflation)
            ));
            ui.label(format!(
                "Interest: {}",
                HumanNumber::from(economy.interest).concise()
            ));
            ui.label(format!(
                "Surplus: {}",
                HumanNumber::from(economy.surplus() as f64).concise()
            ));
            ui.label(format!(
                "Debt: {}",
                HumanNumber::from(economy.debt as f64).concise()
            ));
            ui.label(format!(
                "Money Supply: {}",
                HumanNumber::from(economy.money_supply as f64).concise()
            ));
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

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Econ",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    );
}
