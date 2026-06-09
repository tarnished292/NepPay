mod order;
use order::{create_order, Provider};
mod initiate;
mod esewa;
mod compute;

fn main() {
    // Merchant account
    let merchant_id = "8gBm/:&EnhH.1/q";

    let mut order = create_order(1000, merchant_id.to_string(), Provider::Esewa);
    order.mark_paid();
    println!("{:?}", order);
}
