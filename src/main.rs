use axum::Router;
use db::connect_db;
use dotenvy::dotenv;
use routes::user::user_routes;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod db;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = connect_db().await;

    let app = Router::new()
        .merge(user_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
