use crate::models::{
  InitInventoryDto,QueueStatusDto,ReserveItemDto,SimpleMessageDto,WaitlistNoticeDto
};
use crate::state::{SharedInventory,TicketReservation};
use axum ::{Json,extract::State, http::StatusCode,response::IntoResponse};
use chrono::{Duration,Utc};
use uuid:Uuid;
use crate::models::ReserveSuccessDto;


//Post /init

pub async fn configure_invertory(
    State(inventory_state):State<SharedInventory>,
    Json(payload):Json<InitInventoryDto>,
) ->impl IntoResponse{
    let mut data = inventory_state.lock().unwrap();
    data.stock_count = payload.count;
    data.active_reservations.clear();
    data.waitlist_queue.clear();

    let response = SimpleMessageDto{
        message:format!("Successfully initiated inventory with {} items.",payload.count),
    };
     (StatusCode::OK,Json(response))
    }



// POST /reserve
pub async fn create_reservation(
    State(inventory_state): State<SharedInventory>,
    Json(payload): Json<ReserveItemDto>,
) -> impl IntoResponse {
    let mut data = inventory_state.lock().unwrap();
    let current_time = Utc::now();

    // 1. Purge expired reservations (Mock background job logic)
    let initial_res_count = data.active_reservations.len();
    data.active_reservations.retain(|res| res.expiration_time > current_time);
    let expired_amount = (initial_res_count - data.active_reservations.len()) as u32;

    // 2. Reclaim inventory and process waitlist if stock was freed
    data.stock_count += expired_amount;
    while data.stock_count > 0 {
        if data.waitlist_queue.pop_front().is_some() {
            data.stock_count -= 1;
        } else {
            break;
        }
    }

    // 3. Handle the new incoming request
    if data.stock_count > 0 {
        data.stock_count -= 1;
        
        let new_reservation = TicketReservation {
            id: format!("res_{}", Uuid::new_v4()),
            client_id: payload.user_id.clone(),
            expiration_time: current_time + Duration::minutes(5),
        };

        let response = ReserveSuccessDto {
            reservation_id: new_reservation.id.clone(),
            expires_at: new_reservation.expiration_time,
        };

        data.active_reservations.push(new_reservation);

        (StatusCode::CREATED, Json(ReserveSuccessDto::from(response)).into_response())
    } else {
        // Sold out scenario -> Waitlist
        data.waitlist_queue.push_back(payload.user_id);
        let queue_position = data.waitlist_queue.len();
        
        let response = WaitlistNoticeDto {
            message: "Inventory sold out. You have been added to the waitlist.".to_string(),
            waitlist_position: queue_position,
        };

        (StatusCode::ACCEPTED, Json(response).into_response())
    }
}

//Get status 

pub async fn check_status(State(inventory_state): State<SharedInventory>) -> impl IntoResponse {
    let data = inventory_state.lock().unwrap();

    let response = QueueStatusDto {
        available_stock: data.stock_count,
        current_waitlist_size: data.waitlist_queue.len(),
    };

    (StatusCode::OK, Json(response))
}
