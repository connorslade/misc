use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BoatInfo {
    state: String,
    name: Option<String>,
    reg_to: Option<String>,
    reg_address: Option<String>,
    hid: Option<String>,
    port: Option<String>,
    manufacture: Option<String>,
    year: Option<String>,
    boat_type: Option<String>,
    weight: Option<String>,
    length: Option<String>,
    id: Option<String>,
}

impl BoatInfo {
    pub fn from_raw(state: &str, raw: Vec<String>) -> Self {
        Self {
            state: state.to_owned(),
            name: raw.get(0).cloned(),
            reg_to: raw.get(1).cloned(),
            reg_address: raw.get(2).cloned(),
            hid: raw.get(3).cloned(),
            port: raw.get(4).cloned(),
            manufacture: raw.get(5).cloned(),
            year: raw.get(6).cloned(),
            boat_type: raw.get(7).cloned(),
            weight: raw.get(8).cloned(),
            length: raw.get(9).cloned(),
            id: raw.get(10).cloned(),
        }
    }
}
