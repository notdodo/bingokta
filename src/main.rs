use rust_demo::{infrastructure::web::start_server::start_server, init_otel};

#[tokio::main]
async fn main() {
    let _guard = init_otel();
    tracing::info!("Application started");
    start_server().await;
}