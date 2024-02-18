use ohkami_framework_benchmarks::{load_env, SetServer, ConnectionPool};
use ohkami::{Ohkami, Response, Route};


load_env! {
    DATEBASE_URL  : String,
    MAX_CONNECTIONS: u32,
    MIN_CONNECTIONS: u32,
}

#[tokio::main]
async fn main() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS())
        .min_connections(MIN_CONNECTIONS())
        .connect(&DATEBASE_URL()).await
        .unwrap();

    Ohkami::with((SetServer, ConnectionPool(pool)), (
        "/plaintext".GET(plaintext),
        "/json"     .GET(json),
    )).howl("0.0.0.0:8000").await
}


#[inline(always)]
async fn plaintext() -> &'static str {
    "Hello, World!"
}

#[inline(always)]
async fn json() -> Response {
    Response::OK().json_str(r#"{"message":"Hello, World!"}"#)
}
