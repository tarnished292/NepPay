use crate::{compute::compute_hmac, domain::intent::PaymentIntent};
use serde::Serialize;

#[derive(Serialize)]
pub struct EsewaPayload {
    amount: u64,
    tax_amount: u64,
    product_service_charge: u64,
    product_delivery_charge: u64,
    product_code: String,      //Merchant code will be EPAYTEST :)
    total_amount: u64,
    transaction_uuid: String,
    success_url: String,
    failure_url: String,
    signed_field_names: String,
    signature: String,
}


pub struct Merchant {
    pub product_code: String, // EPAYTEST
    pub secret_key: String,   // 8gBm/:&EnhH.1/q
    // pub success_url: String,
    // pub failure_url: String,
}

pub fn esewa_payload(intent: &PaymentIntent, merchant: &Merchant) -> EsewaPayload {
    let tax_amount = 0;
    let product_service_charge = 10;
    let product_delivery_charge = 0;

    let total_amount = intent.amount + tax_amount + product_service_charge + product_delivery_charge;
    
    let signed_field_names = "total_amount,transaction_uuid,product_code".to_string();

    let message = format!(
        "total_amount={},transaction_uuid={},product_code={}",
        total_amount, intent.id, merchant.product_code
    );

    let signature = compute_hmac(&merchant.secret_key, &message);

    EsewaPayload {
        amount: intent.amount,
        tax_amount: tax_amount,
        product_service_charge: product_service_charge,
        product_delivery_charge: product_delivery_charge,
        product_code: merchant.product_code.clone(),
        total_amount: total_amount,
        transaction_uuid: intent.id.to_string(),
        success_url: intent.success_url.clone(),
        failure_url: intent.failure_url.clone(),
        signed_field_names: signed_field_names,
        signature: signature,
    }
}

pub async fn send_to_esewa(payload: EsewaPayload) -> String {
    let client = reqwest::Client::new();
    let url = "https://rc-epay.esewa.com.np/api/epay/main/v2/form";

    let res = client
        .post(url)
        .form(&payload)
        .send()
        .await
        .expect("Failed to send");

    res.url().to_string()
}
    