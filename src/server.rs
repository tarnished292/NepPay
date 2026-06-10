use axum::{Router, routing::{post}};
use crate::handler::payment::{create_payment, webhook_handler};

pub fn create_router() -> Router{
    Router::new()
        .route("/payment", post(create_payment))
        .route("/webhook/esewa", post(webhook_handler))
}