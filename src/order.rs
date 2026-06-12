// use chrono::{DateTime, Utc};
// use serde::Deserialize;

// #[derive(Debug)]
// pub struct Order {
//     pub id: String,
//     pub amount: u32,
//     pub merchant_id: String,
//     pub status: Status,
//     pub created_at: DateTime<Utc>,
// }

// impl Order {
//     pub fn mark_paid(&mut self) {
//         self.status = Status::Paid;
//     }

//     pub fn mark_failed(&mut self) {
//         self.status = Status::Cancelled;
//     }
// }

// #[derive(Debug)]
// pub enum Status {
//     Pending,
//     Paid,
//     Cancelled,
//     Refunded,
// }



// pub fn create_order(id: String, amount: u32, merchant: String) -> Order {
//     Order {
//         id,
//         amount,
//         merchant_id: merchant,
//         status: Status::Pending,
//         created_at: Utc::now(),
//     }
// }
