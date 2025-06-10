use crate::shipment::{Package, Shipment, ShipmentStatus};
use std::collections::HashMap;

pub struct ShipmentManager {
    shipments: HashMap<String, Shipment>,
}

impl ShipmentManager {
    pub fn new() -> Self {
        ShipmentManager {
            shipments: HashMap::new(),
        }
    }

    pub fn create_shipment(
        &mut self,
        status: ShipmentStatus,
        destination: String,
        time_of_departure: Option<chrono::DateTime<chrono::Utc>>,
        time_of_arrival: Option<chrono::DateTime<chrono::Utc>>,
        shipment_id: Option<String>,
    ) -> &mut Shipment {
        let id = shipment_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let shipment = Shipment::new(
            status,
            destination,
            time_of_departure,
            time_of_arrival,
            id.clone(),
        );
        self.shipments.insert(id.clone(), shipment);
        self.shipments.get_mut(&id).unwrap()
    }

    pub fn get_shipment(&mut self, shipment_id: &str) -> Option<&mut Shipment> {
        self.shipments.get_mut(shipment_id)
    }

    /// List all shipments, with optional status filter.
    pub fn list_shipments(&self, status_filter: Option<ShipmentStatus>) -> Vec<&Shipment> {
        self.shipments
            .values()
            .filter(|s| {
                if let Some(ref status) = status_filter {
                    &s.status == status
                } else {
                    true
                }
            })
            .collect()
    }
}
