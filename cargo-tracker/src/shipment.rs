use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Shipment {
    pub shipment_id: String,
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
        time_of_arrival: Option<DateTime<Utc>>,
        shipment_id: String,
    ) -> Self {
        Self {
            packages: Vec::new(),
            shipment_id,
            destination,
            status,
            time_of_departure,
            time_of_arrival,
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }

    pub fn remove_package(&mut self, package_id: Uuid) -> Option<Package> {
        if let Some(pos) = self.packages.iter().position(|p| p.id == package_id) {
            Some(self.packages.remove(pos))
        } else {
            None
        }
    }

    pub fn update_package(&mut self, package_id: Uuid, new_description: String) -> bool {
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.id == package_id) {
            pkg.description = new_description;
            true
        } else {
            false
        }
    }

    pub fn update_status(&mut self, status_str: &str) -> bool {
        match status_str {
            "Pending" => self.status = ShipmentStatus::Pending,
            "InTransit" => self.status = ShipmentStatus::InTransit,
            "Delivered" => self.status = ShipmentStatus::Delivered,
            "Lost" => self.status = ShipmentStatus::Lost,
            _ => return false,
        }
        true
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
