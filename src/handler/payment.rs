use axum::http::StatusCode;
use axum::{Json, extract::Query, response::Redirect};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};

use crate::{
    compute::compute_hmac,
    esewa::{Merchant, esewa_payload, send_to_esewa},
    initiate::initiate_payment,
    order::{Provider, create_order},
};

#[derive(Serialize)]
pub struct PaymentResponse {
    pub payment_url: String,
}

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub amount: u32,
    pub merchant_id: String,
    pub provider: Provider,
}

pub async fn create_payment(Json(body): Json<PaymentRequest>) -> Json<PaymentResponse> {
    let merchant = Merchant {
        product_code: "EPAYTEST".to_string(),
        secret_key: "8gBm/:&EnhH.1/q".to_string(),
        success_url: "http://localhost:3000/webhook/esewa".to_string(),
        failure_url: "https://developer.esewa.com.np/failure".to_string(),
    };

    let order = create_order(body.amount, body.merchant_id.to_string());
    let intent = initiate_payment(&order, body.provider);
    let payload = esewa_payload(&intent, &order, &merchant);
    let payment_url = send_to_esewa(payload).await;
    println!("{}", payment_url);
    Json(PaymentResponse { payment_url })
}

#[derive(Debug, Deserialize)]
pub struct EsewaWebhookData {
    pub status: String,
    pub transaction_code: String,
    pub total_amount: String,
    pub transaction_uuid: String,
    pub product_code: String,
    pub signature: String,
    pub signed_field_names: String,
}

#[derive(Deserialize)]
pub struct EsewaCallback {
    pub data: String,
}

pub async fn webhook_handler(Query(params): Query<EsewaCallback>) -> StatusCode {
    let decoded = general_purpose::STANDARD
        .decode(params.data)
        .expect("invalid base64");

    let payload: EsewaWebhookData = serde_json::from_slice(&decoded).expect("invalid json");

    let message = format!(
        "transaction_code={},status={},total_amount={},transaction_uuid={},product_code={},signed_field_names={}",
        payload.transaction_code,
        payload.status,
        payload.total_amount,
        payload.transaction_uuid,
        payload.product_code,
        payload.signed_field_names
    );

    println!("our message: {}", message);

    let expected = compute_hmac("8gBm/:&EnhH.1/q", &message);

    if expected != payload.signature {
        return StatusCode::UNAUTHORIZED;
    }
    StatusCode::OK
}
