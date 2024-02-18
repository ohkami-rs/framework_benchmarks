use ohkami_framework_benchmarks::{load_env, SetServer, ConnectionPool, Message, World};
use ohkami::{Ohkami, Route, Memory};
use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};
use sqlx::PgPool;

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
        "/json".GET(json_serialization),
        "/db"  .GET(single_database_query),
    )).howl("0.0.0.0:8000").await
}

#[inline]
async fn json_serialization() -> Message {
    Message {
        message: "Hello, World!"
    }
}

async fn single_database_query(pool: Memory<'_, PgPool>) -> World {
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();

    let world: World = sqlx::query_as(
        "SELECT id, radomNumber FROM World WHERE id = $1")
        .bind((rng.gen::<u32>() % 10_000 + 1) as i32)
        .fetch_one(*pool).await
        .expect("Failed to fetch a world");

    world
}

async fn multiple_database_query(pool: Memory<'_, PgPool>)
