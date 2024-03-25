mod routes;
mod handlers;
mod models;
mod auth;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    // can change where this comes from later
    let db_route = "postgresql://localhost:5432/rust_messages";

    let pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(db_route).await
      .expect("Failed to create pool");
    let shared_pool = Arc::new(pool);

    let routes = routes::routes(shared_pool.clone())
      .with(warp::log("my_api"));
    

    println!("Server started at http://localhost:8001");
    warp::serve(routes).run(([127,0,0,1], 8001)).await;
}
