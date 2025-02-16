#[derive(Default, Clone)]
pub struct LangModule {
    pub labels: [&'static str; 10],
    pub lang_profile: LangProfile,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LangProfile {
    #[default]
    English,
    Español,
}

impl LangModule {
    pub fn set_lang(&mut self) {
        match self.lang_profile {
            LangProfile::English => {
                self.labels = LABELS_EN;
            }
            LangProfile::Español => {
                self.labels = LABELS_ESP;
            }
        }
    }
}

pub const LABELS_EN: [&str; 10] = [
    "Home",
    "New",
    "Open",
    "Close All",
    "Submit",
    "Image",
    "Text",
    "Name",
    "Directory",
    "Kind",
];

pub const LABELS_ESP: [&str; 10] = [
    "Casa",
    "Nuevo",
    "Abierto",
    "Cerrar Todo",
    "Entregar",
    "Imagen",
    "Texto",
    "Nombre",
    "Directorio",
    "Clasificar",
];
