use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub amount: u32,
    provider: Provider,
    merchant_id: String,
    status: Status,
    created_at: DateTime<Utc>,
}

impl Order {
    pub fn mark_paid(&mut self) {
        self.status = Status::Processing;
    }

    pub fn mark_failed(&mut self) {
        self.status = Status::Cancelled;
    }
}

#[derive(Debug)]
pub enum Status {
    Pending,
    Processing,
    Delivered,
    Cancelled,
    Refunded,
}

#[derive(Debug)]
pub enum Provider {
    Esewa,
    Khalti,
    Fonepay,
}

pub fn create_order(amount: u32, merchant: String, provider: Provider) -> Order {
    let random_id = Uuid::new_v4();

    Order {
        id: random_id,
        amount,
        provider,
        merchant_id: merchant,
        status: Status::Pending,
        created_at: Utc::now(),
    }
}
