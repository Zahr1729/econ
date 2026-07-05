use eframe::egui::{self, Ui};
use std::sync::{Arc, Mutex};

use crate::{
    model::{Economy, demographics::Demographics, state::State},
    ui::UiHandler,
};

pub mod model;
pub mod ui;

struct MyApp {
    _economy: Arc<Mutex<Economy>>,
    ui_handler: UiHandler,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut economy = Economy {
            state: State {
                inflation: 0.01,
                interest: 0.02,
                debt: 3_000_000_000_000,
                taxes: 80_000_000_000,
                borrowing: 20_000_000_000,
                spending: 117_000_000_000,
                printing: 17_000_000_000,
                money_supply: 40_000_000_000_000,
                interest_payments: 0,
            },
            demographics: Demographics {},
        };
        economy.state.adjust_interest_payments();
        let econ_wrapper = Arc::new(Mutex::new(economy));
        Self {
            _economy: econ_wrapper.clone(),
            ui_handler: UiHandler::new(econ_wrapper),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.ui_handler.update(ui);
    }
}

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    let _ = eframe::run_native("Econ", options, Box::new(|_cc| Ok(Box::<MyApp>::default())));
}
