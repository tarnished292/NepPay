use uuid::Uuid;

#[derive(Debug)]
pub struct Order {
    id: Uuid,
    amount: u32,
    provider: Provider,
    merchant_id: String,
    status: Status,
}

impl Order {
    pub fn mark_paid(&mut self) {
        self.status = Status::Paid;
    }

    pub fn mark_failed(&mut self) {
        self.status = Status::FAILED;
    }
}

#[derive(Debug)]
enum Status {
    PENDING,
    PROCESSING,
    PAID,
    FAILED,
    EXPIRED,
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
        status: Status::PENDING,
    }
}
