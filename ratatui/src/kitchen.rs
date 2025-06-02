use crate::orders::{Order, Orders};

pub struct Kitchen {
    pub current_orders: Orders,
}

impl Kitchen {
    pub fn new(current_orders: Orders) -> Self {
        Kitchen { current_orders }
    }

    pub fn show_all_orders(&self) {
        self.current_orders.print_all();
    }

    pub fn complete_order(&mut self, table_id: usize) -> Option<Order> {
        self.current_orders
            .remove_order_by_table_id(table_id as u32)
    }
}
