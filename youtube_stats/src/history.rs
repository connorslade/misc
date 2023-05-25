use std::hash::{Hash, Hasher};

use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Watch {
    pub header: String,
    pub title: String,
    pub title_url: String,
    pub subtitles: Vec<Subtitle>,
    pub time: String,
    pub products: Vec<String>,
    pub activity_controls: Vec<String>,
}

#[derive(Deserialize)]
pub struct Subtitle {
    pub name: String,
    pub url: String,
}

impl Watch {
    pub fn id(&self) -> &str {
        self.title_url
            .split(['=', '\u{003d}'])
            .last()
            .unwrap_or_default()
    }
}

impl Eq for Watch {}
impl PartialEq for Watch {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Hash for Watch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
