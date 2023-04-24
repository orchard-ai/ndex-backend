pub mod app_state;
pub mod models;
mod router;
mod routes;
pub mod utilities;

use app_state::AppState;
use router::create_router;

use anyhow::{self, Ok};
use std::net::SocketAddr;

pub async fn run(app_state: AppState) -> anyhow::Result<()> {
    let app = create_router(app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], 3001));

    tracing_subscriber::fmt::init();

    tracing::debug!("Listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
