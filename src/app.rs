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
    pub show_home_tab_on_startup: bool,
    pub selected_tab: usize,
    pub previous_selected_tab: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TabKey {
    #[default]
    Home,
    DocumentTab(Document),
}

impl TabKey {
    fn init_doc(&mut self) {
        match self {
            TabKey::Home => {}
            TabKey::DocumentTab(document) => document.init = true,
        };
    }
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
    pub fn pick_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
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
    pub fn show_home_tab(&mut self) {
        if !self.tabs.contains(&TabKey::Home) {
            self.tabs.push(TabKey::Home);
            self.previous_selected_tab = self.selected_tab;
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
        self.tabs.push(TabKey::DocumentTab(doc));
        self.previous_selected_tab = self.selected_tab;
        self.selected_tab = self.tabs.len() - 1;
    }
    pub fn pick_file(&mut self) {
        if let Some(_path) = rfd::FileDialog::new()
            .pick_file()
            .map(std::path::PathBuf::from)
        {};
    }
    pub fn close_all(&mut self) {
        self.tabs.clear();
    }
    pub fn remove_tab(&mut self, i: usize) {
        if self.previous_selected_tab != 0 {
            self.selected_tab = self.previous_selected_tab - 1;
        } else {
            self.selected_tab = 0;
        }
        self.previous_selected_tab = self.selected_tab;
        self.tabs.remove(i);
    }
    pub fn init_doc(&mut self) {
        self.tabs[self.selected_tab].init_doc();
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
