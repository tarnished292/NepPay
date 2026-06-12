use axum::http::StatusCode;
use axum::{Json, extract::Query};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};

use crate::domain::intent::Provider;
use crate::{
    compute::compute_hmac,
    domain::intent::initiate_payment,
    providers::esewa::{Merchant, esewa_payload, send_to_esewa},
};

#[derive(Serialize)]
pub struct PaymentResponse {
    pub payment_url: String,
}

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub amount: u64,
    pub merchant_order_id: String, //merchant product order id
    pub merchant_id: String,       // nep pay website give unique merchant id to their mechant
    pub provider: Provider,
    pub success_url: String,
    pub failure_url: String,
}

pub async fn create_payment(Json(body): Json<PaymentRequest>) -> Json<PaymentResponse> {
    let merchant = Merchant {
        product_code: "EPAYTEST".to_string(),
        secret_key: "8gBm/:&EnhH.1/q".to_string(),
        // merchant_id: "NepPayTest".to_string(),      // Nep pay provide
        // success_url: body.success_url,
        // failure_url: body.failure_url,
    };

    let intent = initiate_payment(
        body.provider,
        body.merchant_order_id,
        body.merchant_id,
        body.amount,
        body.success_url,
        body.failure_url
    );
    let payload = esewa_payload(&intent, &merchant);
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
