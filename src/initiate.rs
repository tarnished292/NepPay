use crate::order::{Order, Provider, Status};
use chrono::{DateTime, Utc};
use uuid::Uuid;

enum IntentStatus {
    Initiated,
    Pending,
    Verified,
    Failed,
    Expired,
}

pub struct PaymentIntent {
    pub id: Uuid,
    pub order_id: Uuid,
    provider_txn_id: Option<String>,
    provider: Provider,
    state: IntentStatus,
    created_at: DateTime<Utc>,
}

pub fn initiate_payment(order: &Order, provider: Provider) -> PaymentIntent {
    let random_id = Uuid::new_v4();

    PaymentIntent {
        id: random_id,
        order_id: order.id,
        provider_txn_id: None,
        provider,
        state: IntentStatus::Initiated,
        created_at: Utc::now(),
    }
}
