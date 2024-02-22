mod routes;
mod handlers;
mod models;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let routes = routes::routes();
    let db_route = "postgresql://localhost:5432/rust_messages";

    let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(db_route).await.unwrap();

    println!("Server started at http://localhost:8001");
    warp::serve(routes).run(([127,0,0,1], 8001)).await;
}
