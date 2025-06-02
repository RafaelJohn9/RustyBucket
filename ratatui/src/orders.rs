use crate::menu::Dish;

impl Orders {
    pub fn new() -> Self {
        Orders { orders: Vec::new() }
    }

    pub fn add_order(&mut self, order: Order) {
        if self.orders.len() > 5 {
            println!("⚠️ Chef, we’re at MAXIMUM OVERLOAD!");
            println!("5+ orders are pending! This is not a drill!");
            println!("Linguini has fainted. Colette is sharpening knives.");
            println!("Suggest: “Advance Order Status” or hide in the pantry.");
        } else {
            self.orders.push(order);
        }
    }

    pub fn update_status(&mut self, table: u32) -> bool {
        for o in &mut self.orders {
            if o.table == table {
                o.status = match o.status {
                    OrderStatus::Pending => OrderStatus::Cooking,
                    OrderStatus::Cooking => OrderStatus::Served,
                    OrderStatus::Served => OrderStatus::Oops,
                    OrderStatus::Oops => OrderStatus::Oops,
                };
                return true;
            }
        }
        false
    }

    pub fn remove_order_by_table_id(&mut self, table: u32) -> bool {
        if let Some(pos) = self.orders.iter().position(|o| o.table == table) {
            self.orders.remove(pos);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Cooking,
    Served,
    Oops,
}

#[derive(Debug)]
pub struct Order {
    pub table: u32,
    pub dish: Dish,
    pub status: OrderStatus,
}

pub struct Orders {
    pub orders: Vec<Order>,
}
