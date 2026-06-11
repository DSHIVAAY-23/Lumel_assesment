



use chrono::{DateTime,Utc};
use std::collections::VecDeque;
use std::sync::{Arc,Mutex};

#[derive(Clone)]
pub struct TicketReservation{
    pub id:String,
    pub client_id:String,
    pub expiration_time:DateTime<Utc>,

}
pub struct InventoryData{
    pub stock_count:u32,
    pub active_reservations:Vec<TicketReservation>,
    pub waitlist_queue:VecDeque<String>,
}

pub type SharedInventory = Arc<Mutex<InventoryData>>;

//Helper to bootstrap our concurrent state map

pub fn initialize_shared_state()->SharedInventory{
    Arc::new(Mutex::new(InventoryData{
        stock_count:0,
        active_reservations:Vec::new(),
        waitlist_queue:VecDeque::new(),
    }))
}