pub mod app_state;
pub mod models;
mod router;
mod routes;
pub mod utilities;

use app_state::AppState;
use router::create_router;

use anyhow::{self, Ok};
use std::{env, net::SocketAddr};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub async fn run(app_state: AppState) -> anyhow::Result<()> {
    let app = create_router(app_state);
    let port_str = env::var("PORT").unwrap_or_else(|_| String::from("3001"));
    let port: u16 = port_str.parse().expect("PORT must be a number");
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
