use sqlx::FromRow;

#[derive(FromRow)]
struct User {
    id: i32,
    name: String,
}
