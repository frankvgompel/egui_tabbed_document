use crate::{app::{App, Document, DocumentKind, TabKey}, language_labels::{LangModule, LangProfile}};
use eframe::egui::{self, Ui};

pub(crate) fn main_interface(app: &mut App, ctx: &egui::Context) {
    let labels = app.language.labels;
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(labels[0], |ui| {
                if ui.button(labels[3]).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(16.0);
            egui::widgets::global_theme_preference_buttons(ui);
            ui.add_space(30.);
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", app.language.lang_profile))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut app.language.lang_profile, LangProfile::English, "English")
                        .clicked()
                    {
                        app.language.set_lang()
                    };
                    if ui
                        .selectable_value(&mut app.language.lang_profile, LangProfile::Español, "Español")
                        .clicked()
                    {
                        app.language.set_lang()
                    };
                });
        });
    });
    egui::TopBottomPanel::top("utility buttons").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button(labels[0]).clicked() {
                // "Home"
                app.show_home_tab();
            }
            if ui.button(labels[1]).clicked() {
                // "New"
                app.add_new_tab();
            }
            if ui.button(labels[2]).clicked() {
                // "Open"
                app.pick_file()
            }
            if ui.button(labels[3]).clicked() {
                // "Close"
                app.close_all()
            }
        });
    });
    egui::TopBottomPanel::top("tab bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for (i, tab) in app.tabs.clone().iter().enumerate() {
                match tab {
                    TabKey::Home => {
                        if ui.label(labels[0]).clicked() {
                            // "Home"
                            app.selected_tab = i
                        }
                    }
                    TabKey::DocumentTab(_) => {
                        if ui.label(labels[1]).clicked() {
                            // "New"
                            app.selected_tab = i
                        }
                    } // TabKey::DocumentTab(document) => {
                      //     match document.kind {
                      //         crate::app::DocumentKind::Text => if ui.label(app.labels[1]).clicked() { // "new"
                      //             app.selected_tab = i
                      //         },
                      //         crate::app::DocumentKind::Image => if ui.label(app.labels[1]).clicked() { // "new"
                      //             app.selected_tab = i
                      //         },
                      //     }
                      // },
                };
                if ui.selectable_label(true, "ｘ").clicked() {
                    app.remove_tab(i);
                }
                ui.add_space(10.);
            }
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        if !app.tabs.is_empty() {
            match app.tabs[app.selected_tab] {
                TabKey::Home => show_home_ui(ui, &mut app.show_home_tab_on_startup),
                TabKey::DocumentTab(ref mut doc) => match doc.init {
                    true => show_text_or_image(ui, doc),
                    false => show_form(ui, doc, &app.language),
                },
            }
        }
    });
}

fn show_home_ui(ui: &mut Ui, checked: &mut bool) {
    ui.label("Home Page");
    ui.checkbox(checked, "Show Home on startup");
}
fn show_form(ui: &mut Ui, doc: &mut Document, language: &LangModule) {
    let labels = language.labels;
    ui.add_space(10.);
    egui::Grid::new("Form")
        .spacing(egui::Vec2::new(10., 10.))
        .show(ui, |ui| {
            ui.label(labels[7]); // "Name"
            ui.text_edit_singleline(&mut doc.name);
            ui.end_row();

            ui.label(labels[8]); // "Directory"
            ui.label(doc.path.clone().into_boxed_path().to_string_lossy());
            if ui.button("...").clicked() {
                doc.pick_file();
            };
            ui.end_row();
            ui.label(labels[9]); // "Kind"
            egui::ComboBox::from_id_salt(format!("{}", doc.name))
                .selected_text(doc.kind.fmt(language))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut doc.kind, DocumentKind::Text, labels[6]);
                    ui.selectable_value(&mut doc.kind, DocumentKind::Image, labels[5]);
                });
            ui.end_row();

            ui.label("");
            if ui.button(labels[4]).clicked() {
                // "Submit"
                doc.init_doc();
                dbg!(&doc);
            }
            ui.end_row();
        });
}
fn show_text_or_image(ui: &mut Ui, doc: &mut Document) {
    match doc.kind {
        DocumentKind::Text => ui.label("Text Document"),
        DocumentKind::Image => ui.label("Image Document"),
    };
}
