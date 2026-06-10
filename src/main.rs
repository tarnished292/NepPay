use tokio::net::TcpListener;


mod order;
mod compute;
mod esewa;
mod server;
mod initiate;
mod handler;

#[tokio::main]
async fn main() {
    let app = server::create_router();
    let listenser = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("NepPay running on http://localhost:3000");
    axum::serve(listenser, app).await.unwrap();
}

