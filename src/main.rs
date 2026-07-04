use std::sync::{Arc, Mutex};

use egui::{self};

use crate::{
    model::{Economy, demographics::Demographics, state::State},
    ui::UiHandler,
};

pub mod model;
pub mod ui;

struct MyApp {
    economy: Arc<Mutex<Economy>>,
    ui_handler: UiHandler,
}

impl Default for MyApp {
    fn default() -> Self {
        let economy = Economy {
            state: State {
                inflation: 0.01,
                interest: 0.02,
                debt: 3_000_000_000_000,
                taxes: 80_000_000_000,
                borrowing: 20_000_000_000,
                spending: 117_000_000_000,
                printing: 17_000_000_000,
                money_supply: 40_000_000_000_000,
            },
            demographics: Demographics {},
        };
        let econ_wrapper = Arc::new(Mutex::new(economy));
        Self {
            economy: econ_wrapper.clone(),
            ui_handler: UiHandler::new(econ_wrapper),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui_handler.update(ctx);
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
