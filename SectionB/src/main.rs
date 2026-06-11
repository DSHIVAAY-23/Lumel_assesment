// **Section B: API Systems Design**
// **Systems – API Design – Medium**

// **The "Flash Sale" Inventory Reservation API**

// Problem Statement:
// Design and implement a set of RESTful endpoints for a high-traffic "Flash Sale" system. You are managing the inventory for a single high-demand product (e.g., a concert ticket or limited edition sneaker).
// The system must handle concurrent requests and ensure that we never oversell the inventory.
// **Requirements**:
// 1. **Initialize**: Endpoint to set the total inventory count (Admin only).
// 2. **Reserve**: Endpoint for a user to reserve an item.
//     ◦ If inventory > 0, decrement inventory and return a `reservation_id`.
//     ◦ Reservations expire after 5 minutes if not confirmed (mock this logic or explain how you would handle the expiry in a real system).
//     ◦ If inventory = 0, add the user to a "Waitlist" and return a specific status.
// 3. **Status**: Endpoint to check the current available inventory and waitlist size.
// **Constraints**:
// • Assume the data is stored in memory for this exercise, but code must be thread-safe or handle concurrency conceptually (e.g., using mutexes, atomic operations, or appropriate database locking pseudo-code).
// • Focus on appropriate HTTP status codes (200, 201, 409, 429).
// • Input format: JSON.
// Example Request:
// POST /reserve
// Body: { "user_id": "user_123" }
// Example Response (Success):
// 201 Created
// Body: { "reservation_id": "res_555", "expires_at": "2023-10-27T10:05:00Z" }
// Example Response (Sold Out):
// 202 Accepted
// Body: { "message": "Added to waitlist", "waitlist_position": 45 }
mod handlers;
mod models;
mod state;
use std::net::TcpListener;

use axum::{
   routing::{get, post},
     Router,
};
use handlers::{check_status, configure_inventory, create_reservation};
use state::initialize_shared_state;

#[tokio::main]
async fn main() {
    // Initialize the in-memory shared state
    let app_state = initialize_shared_state();

    // Define the application routes
    let api_router = Router::new()
        .route("/init", post(configure_inventory))
        .route("/reserve", post(create_reservation))
        .route("/status", get(check_status))
        .with_state(app_state);

    // Bind to the port
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    println!(" Flash Sale API server listening on http://localhost:3000");
    axum::serve(listener, api_router).await.unwrap();
}