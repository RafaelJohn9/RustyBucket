use crate::orders::{Order, Orders};

pub struct Kitchen {
    pub current_orders: Orders,
}

impl Kitchen {
    pub fn new(current_orders: Orders) -> Self {
        Kitchen { current_orders }
    }

    pub fn prepare_order(&mut self, order: Order) {
        self.current_orders.add_order(order); // Use Orders' method
    }

    pub fn complete_order(&mut self, table_id: usize) -> Option<Order> {
        self.current_orders.remove_order_by_table_id(table_id)
    }
}
