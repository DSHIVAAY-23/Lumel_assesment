use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};
#[derive(Deserialize)]
pub struct InitInventoryDto{
    pub count:u32,
}

#[derive(Deserialize)]
pub struct ReserveItemDto{
    pub user_id:String,

}
#[derive(Serialize)]
pub struct ReserveSuccessDto{
    pub reservation_id:String,
    pub expires_at:DateTime<Utc>,
}

#[derive(Serialize)]
pub struct WaitlistNoticeDto{
    pub message:String,
    pub waitlist_position:usize,
}

#[derive(Serialize)]

pub struct QueueStatusDto{
    pub available_stock:u32,
    pub current_waitlist_size:usize,
}

#[derive(Serialize)]

pub struct SimpleMessageDto{
    pub message:String,
}

