use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
}

// Fetch users from the database
async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users");

    Json(users)
}

// Define routes (no need to pass pool)
pub fn user_routes() -> Router<PgPool> {
    Router::new().route("/api/users", get(get_users)) // Route with handler
}
