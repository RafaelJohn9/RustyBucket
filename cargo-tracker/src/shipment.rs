use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug)]
pub struct Shipment {
    pub tracking_id: String,
    pub destination: String,
    pub packages: Vec<Package>,
    pub status: ShipmentStatus,
    pub time_of_departure: Option<DateTime<Utc>>,
    pub time_of_arrival: Option<DateTime<Utc>>,
}

impl Shipment {
    pub fn new(
        status: ShipmentStatus,
        destination: String,
        time_of_departure: Option<DateTime<Utc>>,
        tracking_id: String,
    ) -> Self {
        Self {
            packages: Vec::new(),
            tracking_id,
            destination,
            status,
            time_of_departure,
            time_of_arrival: None,
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    pub fn to_json_str(&self) -> String {
        let packages_json = self
            .packages
            .iter()
            .map(|p| {
                json!({
                    "id": p.id.to_string(),
                    "description": p.description,
                })
            })
            .collect::<Vec<_>>();

        let json_obj = json!({
            "tracking_id": self.tracking_id,
            "destination": self.destination,
            "status": format!("{:?}", self.status),
            "time_of_departure": self.time_of_departure.map(|t| t.to_rfc3339()),
            "time_of_arrival": self.time_of_arrival.map(|t| t.to_rfc3339()),
            "packages": packages_json,
        });

        json_obj.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShipmentStatus {
    Pending,
    InTransit,
    Delivered,
    Lost,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub id: Uuid,
    pub description: String,
}

impl Package {
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
        }
    }
}
