use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;


 #[warn(unused)]
enum IntentStatus {
    Pending,
    Paid,
    Failed,
    Expired,
}

#[derive(Debug, Deserialize)]
pub enum Provider {
    Esewa,
    Khalti,
    Fonepay,
}

pub struct PaymentIntent {
    pub id: Uuid,
    pub merchant_order_id: String, //Merchant item Order Id
    pub merchant_id: String,       //Merchant ID to point in nep pay data
    pub amount: u64,
    pub provider: Provider,
    pub status: IntentStatus,
    pub success_url: String,
    pub failure_url: String,
    created_at: DateTime<Utc>,
}

pub fn initiate_payment(
    provider: Provider,
    merchant_order_id: String,
    merchant_id: String,
    amount: u64,
    success_url: String,
    failure_url: String,
) -> PaymentIntent {
    let random_id = Uuid::new_v4();

    PaymentIntent {
        id: random_id,
        merchant_order_id,
        merchant_id,
        amount,
        provider,
        success_url,
        failure_url,
        status: IntentStatus::Pending,
        created_at: Utc::now(),
    }
}
