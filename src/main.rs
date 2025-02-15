mod app;
mod interface;
mod language_labels;
use app::App;

use eframe;
pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Egui Tabbed Documents",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
