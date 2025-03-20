pub mod postgres;
pub mod redis;
pub mod schema;
pub mod routes;
pub mod handlers;
pub mod jobs;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use actix_rt::Arbiter;
use std::time::Duration;
use tokio::time::interval;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let binding_address = std::env::var("BINDING_ADDRESS")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "BINDING_ADDRESS is not set"))?;

    let binding_port: u16 = std::env::var("BINDING_PORT")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "BINDING_PORT is not set"))?
        .parse()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse BINDING_PORT"))?;

    let postgres_url = std::env::var("POSTGRES_URL")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "POSTGRES_URL is not set"))?;

    let redis_url = std::env::var("REDIS_URL")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "REDIS_URL is not set"))?;

    let postgres = web::Data::new(postgres::Postgres::new(&postgres_url));
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    let mut postgres_connection = postgres.get_connection();
    postgres_connection.run_pending_migrations(MIGRATIONS)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let redis = web::Data::new(redis::Redis::new(&redis_url));

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let arbiter = Arbiter::new();
    arbiter.spawn(async {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            match jobs::health::job().await {
                Ok(_) => {},
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(postgres.clone())
            .app_data(redis.clone())
            .wrap(Logger::default())
            .service(health)
            .configure(routes::projects::config)
    })
    .bind((binding_address, binding_port))?
    .run()
    .await?;

    arbiter.stop();

    Ok(())
}