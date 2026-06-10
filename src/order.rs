use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub amount: u32,
    pub merchant_id: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub fn mark_paid(&mut self) {
        self.status = Status::Pending;
    }

    pub fn mark_failed(&mut self) {
        self.status = Status::Cancelled;
    }
}

#[derive(Debug)]
pub enum Status {
    Pending,
    Paid,
    Cancelled,
    Refunded,
}

#[derive(Debug, Deserialize)]
pub enum Provider {
    Esewa,
    Khalti,
    Fonepay,
}

pub fn create_order(amount: u32, merchant: String) -> Order {
    let random_id = Uuid::new_v4();

    Order {
        id: random_id,
        amount,
        merchant_id: merchant,
        status: Status::Pending,
        created_at: Utc::now(),
    }
}
