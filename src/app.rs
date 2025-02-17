#![allow(dead_code)]

use crate::interface::main_interface;
use crate::language_labels::{self, LangModule, LangProfile};
use eframe;
use egui_colors::{utils, Colorix};
use rfd;
use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct App {
    pub colorix: Colorix,
    pub language: LangModule,
    pub tabs: Vec<TabKey>,
    pub tab_names: Vec<String>,
    pub documents: Vec<Document>,
    pub show_home_tab_on_startup: bool,
    pub selected_tab: usize,
    pub previous_tab: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TabKey {
    #[default]
    Home,
    DocumentTab,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum DocumentKind {
    #[default]
    Text,
    Image,
}
impl DocumentKind {
    pub fn fmt(&self, lang: &LangModule) -> String {
        match self {
            Self::Image => lang.labels[5].to_owned(),
            Self::Text => lang.labels[6].to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Document {
    pub init: bool,
    pub name: String,
    pub kind: DocumentKind,
    pub path: PathBuf,
}

impl Document {
    pub fn init_doc(&mut self) {
        self.init = true
    }
    pub fn set_dir(&mut self) {
        self.set_directory();
        // TODO save file
    }
    fn set_directory(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.path = path
        };
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let colorix = Colorix::global(&cc.egui_ctx, utils::OFFICE_GRAY)
            .animated()
            .set_time(1.);
        let mut language = LangModule::default();
        language.labels = language_labels::LABELS_EN;
        language.lang_profile = LangProfile::English;
        Self {
            colorix,
            language,
            ..Default::default()
        }
    }
    pub fn save_dir(&mut self) {
        self.documents[self.selected_tab].set_dir();
    }
    pub fn init_doc(&mut self) {
        self.documents[self.selected_tab].init_doc();
        match self.documents[self.selected_tab].kind {
            DocumentKind::Text => {
                self.tab_names[self.selected_tab] =
                    format!("{}.txt", self.documents[self.selected_tab].name)
            }
            DocumentKind::Image => {
                self.tab_names[self.selected_tab] =
                    format!("{}.bmp", self.documents[self.selected_tab].name)
            }
        }
    }
    pub fn show_home_tab(&mut self) {
        if !self.tabs.contains(&TabKey::Home) {
            self.tabs.push(TabKey::Home);
            self.documents.push(Document::default());
            self.tab_names.push("Home".to_string());
            self.previous_tab = self.selected_tab;
            self.selected_tab = self.tabs.len() - 1;
        } else {
            for (i, v) in self.tabs.iter().enumerate() {
                if *v == TabKey::Home {
                    self.selected_tab = i
                }
            }
        }
    }
    pub fn add_new_tab(&mut self) {
        let mut doc = Document::default();
        doc.name = format!("New {}", self.tabs.len());
        self.tabs.push(TabKey::DocumentTab);
        self.documents.push(Document::default());
        self.tab_names.push(self.language.labels[1].to_string());
        self.previous_tab = self.selected_tab;
        self.selected_tab = self.tabs.len() - 1;
    }
    pub fn pick_file(&mut self) {
        if let Some(file) = rfd::FileDialog::new().pick_file() {
            let mut doc = Document::default();
            doc.path = file.parent().unwrap().to_path_buf();
            let path = file.as_path();
            if let Some(extension) = path.extension() {
                if extension == "txt" {
                    doc.kind = DocumentKind::Text
                } else if extension == "bmp" {
                    doc.kind = DocumentKind::Image
                }
            }
            if let Some(file_stem) = path.file_stem() {
                let name = file_stem.to_os_string().into_string().unwrap();
                self.tab_names.push(name.clone());
                doc.name = name;
            };
            self.tabs.push(TabKey::DocumentTab);
            doc.init = true;
            self.documents.push(doc);
            self.previous_tab = self.selected_tab;
            self.selected_tab = self.tabs.len() - 1;
        };
    }
    pub fn close_all(&mut self) {
        self.tabs.clear();
    }

    pub fn decrease_selected(&mut self) {
        if self.selected_tab != 0 {
            self.selected_tab = self.selected_tab - 1;
        }
    }
    pub fn decrease_previous(&mut self) {
        if self.previous_tab != 0 {
            self.previous_tab = self.previous_tab - 1;
        }
    }
    // pub fn _remove_tab(&mut self, i: usize) {
    //     if self.previous_tab != 0 {
    //         self.selected_tab = self.previous_tab - 1;
    //     } else {
    //         self.selected_tab = 0;
    //     }
    //     self.previous_tab = self.selected_tab;
    //     self.tabs.remove(i);
    //     self.tab_names.remove(i);
    //     self.documents.remove(i);
    // }

    pub fn remove_tab(&mut self, i: usize) {
        if self.selected_tab == i {
            if i < self.previous_tab {
                self.decrease_previous();
            }
            self.selected_tab = self.previous_tab
        }
        else if self.previous_tab == i || self.selected_tab > i {
            self.decrease_previous();
            self.decrease_selected();
        }
        self.tabs.remove(i);
        self.tab_names.remove(i);
        self.documents.remove(i);
    }
}

impl eframe::App for App {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.colorix.set_animator(ctx);
        main_interface(self, ctx)
    }
}
