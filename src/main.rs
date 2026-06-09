mod order;
use order::{Provider, create_order};

use crate::{esewa::{Merchant, esewa_payload, send_to_esewa}, initiate::initiate_payment};

mod compute;
mod esewa;
mod initiate;

#[tokio::main]
async fn main() {
    // Merchant account

    let merchant = Merchant {
        product_code: "EPAYTEST".to_string(),
        secret_key: "8gBm/:&EnhH.1/q".to_string(),
        success_url: "https://nep-pay.vercel.app/".to_string(),
        failure_url: "https://github.com/tarnished292/NepPay".to_string(),
    };
    let order = create_order(999999, "Tarnished_Merchent".to_string());
    let intent = initiate_payment(&order, Provider::Esewa);
    let payload = esewa_payload(&intent, &order, &merchant);
    let result = send_to_esewa(payload).await;
    println!("{}", result);
}
