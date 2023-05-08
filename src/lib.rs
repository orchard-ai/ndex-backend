pub mod app_state;
pub mod models;
mod router;
mod routes;
pub mod utilities;

use app_state::AppState;
use router::create_router;

use anyhow::{self, Ok};
use std::net::SocketAddr;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub async fn run(app_state: AppState) -> anyhow::Result<()> {
    let app = create_router(app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], 3001));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
