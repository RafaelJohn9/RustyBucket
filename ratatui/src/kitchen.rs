use crate::orders::Orders;

pub struct Kitchen {
    pub current_orders: Orders,
}

impl Kitchen {
    pub fn new(current_orders: Orders) -> Self {
        Kitchen { current_orders }
    }
}
