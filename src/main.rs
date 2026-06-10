use tower_http::cors::CorsLayer;

mod order;
mod compute;
mod esewa;
mod server;
mod initiate;
mod handler;

#[tokio::main]
async fn main() {
    let app = server::create_router()
            .layer(CorsLayer::permissive());
    
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();
    println!("NepPay running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

