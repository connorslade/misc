use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BoatInfo {
    state: String,
    name: String,
    manufacture: Option<String>,
    year: Option<u16>,
    boat_type: Option<BoatType>,
    weight: Option<f32>,
    length: Option<f32>,
    id: Option<u32>,
}

impl BoatInfo {
    pub fn from_raw(state: &str, raw: Vec<String>) -> Self {
        Self {
            state: state.to_owned(),
            name: raw[0].to_owned(),
            manufacture: raw.get(5).cloned(),
            year: raw.get(6).map(|x| x.parse().ok()).flatten(),
            boat_type: raw.get(7).map(|x| x.into()),
            weight: raw.get(8).map(|x| x.parse().ok()).flatten(),
            length: raw.get(9).map(|x| x.parse().ok()).flatten(),
            id: raw.get(10).map(|x| x.parse().ok()).flatten(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum BoatType {
    Recreational,
    FreightBarge,
    Passenger,
    Unclassified,
    CommercialFishingVessel,
    TowingVessel,
    IndustrialVessel,
    OffshoreSupplyVessel,
    TankBarge,
    FreightShip,
    PassengerBarge,
    ResearchVessel,
    SchoolShip,
    PublicVessel,
    TankShip,
    OilRecovery,
    FishingTender,
    FishProcessingVessel,
    PublicFreight,
    MobileOffshoreDrillingUnit,
    PublicTankship,
    Unknown,
}

impl From<&String> for BoatType {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "Recreational" => BoatType::Recreational,
            "Freight Barge" => BoatType::FreightBarge,
            "Passenger (Uninspected)" | "Passenger (Inspected)" => BoatType::Passenger,
            "Unclassified" => BoatType::Unclassified,
            "COMMERCIAL FISHING VESSEL" => BoatType::CommercialFishingVessel,
            "TOWING VESSEL" => BoatType::TowingVessel,
            "Industrial Vessel" => BoatType::IndustrialVessel,
            "Offshore Supply Vessel" => BoatType::OffshoreSupplyVessel,
            "Tank Barge" => BoatType::TankBarge,
            "Freight Ship" => BoatType::FreightShip,
            "Passenger Barge  (Uninspected)" | "Passenger Barge  (Inspected)" => {
                BoatType::PassengerBarge
            }
            "Research Vessel" => BoatType::ResearchVessel,
            "School Ship" => BoatType::SchoolShip,
            "Public Vessel, Unclassified" => BoatType::PublicVessel,
            "Tank Ship" => BoatType::TankShip,
            "Oil Recovery" => BoatType::OilRecovery,
            "Fishing Tender" => BoatType::FishingTender,
            "Fish Processing Vessel" => BoatType::FishProcessingVessel,
            "Public Freight" => BoatType::PublicFreight,
            "Mobile Offshore Drilling Unit" => BoatType::MobileOffshoreDrillingUnit,
            "Public Tankship/Barge" => BoatType::PublicTankship,
            _ => BoatType::Unknown,
        }
    }
}
