use egui;
use egui::vec2;
use humanly::HumanNumber;

use crate::{model::Economy, ui::widgets::bar_chart::BarChart};

pub mod model;
pub mod ui;

struct MyApp {
    economy: Economy,
    borrowing_bar: u32,
    spending_bar: u32,
    taxes_bar: u32,
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
            borrowing_bar: 30,
            spending_bar: 100,
            taxes_bar: 100,
            points: (0..100).map(|i| (i + 30) * (i + 10)).collect(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Right Panel").show(ctx, |ui| {
            let economy = &mut self.economy;
            ui.heading("Economy");
            ui.vertical(|ui| {
                ui.label("Taxes");
                let text =
                    HumanNumber::from(self.taxes_bar as f64 * 212_030_100_000.0 / 100.0).concise();
                ui.add(egui::Slider::new(&mut self.taxes_bar, 0..=120).text(format!("{}", text)))
            });
            ui.vertical(|ui| {
                ui.label("Borrowing");
                let text =
                    HumanNumber::from(self.taxes_bar as f64 * 82_71_210_000.0 / 100.0).concise();
                ui.add(
                    egui::Slider::new(&mut self.borrowing_bar, 0..=120).text(format!("{}", text)),
                )
            });

            ui.vertical(|ui| {
                ui.label("Spending");
                let text =
                    HumanNumber::from(self.taxes_bar as f64 * 172_135_100_000.0 / 100.0).concise();
                ui.add(egui::Slider::new(&mut self.spending_bar, 0..=120).text(format!("{}", text)))
            });
            let button = ui.button("Progress Year");
            economy.taxes = self.taxes_bar as u64 * 200_000_000_000 / 100;
            economy.borrowing = self.borrowing_bar as u64 * 80_000_000_000 / 100;
            economy.spending = self.spending_bar as u64 * 200_000_000_000 / 100;
            if button.clicked() {
                economy.progress_year();
            }
            ui.label(format!("Economy: {:?}", economy));
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
