use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};

use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌓 REST API Service 🌓");

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL variable no esta definida");

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Conexion a la base de datos exitosa");
            pool
        }
        Err(err) => {
            println!("❌ Error al conectar a la base de datos: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    println!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status":"ok",
        "message": MESSAGE
    });

    Json(json_response)
}
