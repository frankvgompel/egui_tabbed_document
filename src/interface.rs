use std::fs;
use crate::{
    app::{App, Document, DocumentKind, TabKey},
    language_labels::LangProfile,
};
use eframe::egui::{self, load::SizedTexture, Image, ImageSource, TextureOptions, Ui};
use image::{GenericImageView, ImageReader};

pub fn main_interface(app: &mut App, ctx: &egui::Context) {
    let labels = app.language.labels;
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            app.colorix.light_dark_toggle_button(ui, 12.);
            ui.add_space(10.);
            ui.menu_button(labels[0], |ui| {
                if ui.button(labels[3]).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(30.);
            app.colorix.themes_dropdown(ui, None, false);
            ui.add_space(30.);
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", app.language.lang_profile))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(
                            &mut app.language.lang_profile,
                            LangProfile::English,
                            "English",
                        )
                        .clicked()
                    {
                        app.language.set_lang()
                    };
                    if ui
                        .selectable_value(
                            &mut app.language.lang_profile,
                            LangProfile::Español,
                            "Español",
                        )
                        .clicked()
                    {
                        app.language.set_lang()
                    };
                });
        });
    });
    egui::TopBottomPanel::top("utility buttons").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button(labels[0]).clicked() { // "Home"
                app.show_home_tab();
            }
            if ui.button(labels[1]).clicked() { // "New"
                app.add_new_tab();
            }
            if ui.button(labels[2]).clicked() { // "Open"
                app.pick_file()
            }
            if ui.button(labels[3]).clicked() { // "Close All"
                app.close_all()
            }
        });
    });
    egui::TopBottomPanel::top("tab bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for (i, tab) in app.tabs.clone().iter().enumerate() {
                if app.tabs.get(i).is_some() {
                    let color = if i == app.selected_tab  {
                        app.colorix.animator.animated_tokens.active_ui_element_background()
                    }
                    else {
                        app.colorix.animator.animated_tokens.app_background()
                    };
                    match tab {
                        TabKey::Home => {
                            if ui.label(egui::RichText::new(labels[0]).background_color(color)).clicked() { // "Home"
                                app.update_tabs(i);
                            }
                        }
                        TabKey::DocumentTab => {
                            if ui.label(egui::RichText::new(&app.tab_names[i]).background_color(color)).clicked() { // "New"
                                app.update_tabs(i);
                            }
                        }
                    };
                    if ui.selectable_label(true, "ｘ").clicked() {
                        app.remove_tab(i);
                    }
                    ui.add_space(10.);
                }
            }
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if !app.tabs.is_empty() {
                match app.tabs[app.selected_tab] {
                    TabKey::Home => show_home_ui(ui, &mut app.show_home_tab_on_startup),
                    // TabKey::DocumentTab(ref mut doc) => match doc.init {
                    TabKey::DocumentTab => match app.documents[app.selected_tab].init {
                        true => show_document(ctx, &mut app.documents[app.selected_tab]),
                        false => show_form(app, ui),
                    },
                }
            }
        });
        // if !app.tabs.is_empty() {
        //     match app.tabs[app.selected_tab] {
        //         TabKey::Home => show_home_ui(ui, &mut app.show_home_tab_on_startup),
        //         // TabKey::DocumentTab(ref mut doc) => match doc.init {
        //         TabKey::DocumentTab => match app.documents[app.selected_tab].init {
        //             true => show_document(ctx, &mut app.documents[app.selected_tab]),
        //             false => show_form(app, ui),
        //         },
        //     }
        // }
    });
}

fn show_home_ui(ui: &mut Ui, checked: &mut bool) {
    ui.label("Welcome on the Home Page");
    ui.checkbox(checked, "Show Home on startup");
}
fn show_form(app: &mut App, ui: &mut Ui) {
    let labels = app.language.labels;
    ui.add_space(10.);
    egui::Grid::new("Form")
        .spacing(egui::Vec2::new(10., 10.))
        .show(ui, |ui| {
            ui.label(labels[7]); // "Name"
            ui.text_edit_singleline(&mut app.documents[app.selected_tab].name);
            ui.end_row();

            ui.label(labels[8]); // "Directory"
            ui.label(
                app.documents[app.selected_tab]
                    .path
                    .clone()
                    .into_boxed_path()
                    .to_string_lossy(),
            );
            if ui.button("...").clicked() {
                app.save_dir();
            };
            ui.end_row();

            ui.label(labels[9]); // "Kind"
            egui::ComboBox::from_id_salt(format!("{}", app.documents[app.selected_tab].name))
                .selected_text(app.documents[app.selected_tab].kind.fmt(&app.language))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.documents[app.selected_tab].kind,
                        DocumentKind::Text,
                        labels[6],
                    );
                    ui.selectable_value(
                        &mut app.documents[app.selected_tab].kind,
                        DocumentKind::Image,
                        labels[5],
                    );
                });
            ui.end_row();

            ui.label("");
            if ui.button(labels[4]).clicked() { // "Submit"
                app.init_doc(); // save file
            }
            ui.end_row();
        });
}

fn show_document(ctx: &egui::Context, doc: &mut Document) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.label(&doc.name);
        ui.label(doc.path.as_os_str().to_str().unwrap());
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        let path_str = doc.path.as_os_str().to_str().unwrap();
        match doc.kind {
            DocumentKind::Text => {
                let path = format!("{}/{}.txt", path_str, doc.name);
                if let Ok(mut text) = fs::read_to_string(&path) {
                    ui.text_edit_multiline(&mut text);
                }
            }
            DocumentKind::Image => {
                let path = format!("{}/{}.bmp", path_str, doc.name);
                let img = ImageReader::open(path).ok().unwrap().decode().ok().unwrap();
                let size = img.dimensions();
                let rgba = img.to_rgba8();
                let pixels = rgba.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], pixels.as_slice());
                let handle = ctx.load_texture("loaded_image", color_image, TextureOptions::default());
                let image_source = ImageSource::Texture(SizedTexture::from_handle(&handle));
                let image = Image::new(image_source);
                ui.add(image);
            }
        };
    });
}


