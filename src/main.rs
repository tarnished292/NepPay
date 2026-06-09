struct Order {
    id: String,
    amount: i32,
    status: Status,
}

 #[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Failed,
}

fn main() {
  let order = Order {
      id: String::from("162gghf1"),
      amount: 1000,
      status: Status::Pending,
  };
  println!("ID: {}", order.id);  
  println!("Amount: {}", order.amount);  
  println!("Status: {:?}", order.status);  
}
