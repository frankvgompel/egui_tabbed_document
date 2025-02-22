mod app;
mod interface;
mod language_labels;
use app::App;
use eframe::egui;
use std::sync::Arc;

pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "inter_medium".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/Inter-Medium.otf"
        ))),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "inter_medium".to_owned());
    eframe::run_native(
        "Egui Tabbed Documents",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    )
}
