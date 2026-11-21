mod config;
mod embedded;
mod github;
mod listing;
mod pages;
mod refresh;
mod routes;
mod shutdown;
mod state;

use std::{net::Ipv4Addr, sync::Arc};

use anyhow::{Context, Result};
use arc_swap::ArcSwap;
use tokio::net::TcpListener;
use tracing::Level;

use crate::{
    config::Config,
    github::{GitHub, Repo},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let Config {
        repos,
        github_token,
        listing,
        refresh_period,
        port,
    } = Config::from_env()?;
    let github = GitHub::new(github_token)?;
    let mut repos: Vec<Repo> = repos.into_iter().map(Repo::new).collect();

    refresh::sync(&github, &mut repos).await;
    let state = Arc::new(AppState {
        snapshot: ArcSwap::new(refresh::snapshot(&listing, &repos)?),
        listing,
    });

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, port))
        .await
        .with_context(|| format!("failed to bind port {port}"))?;
    tokio::spawn(refresh::run(
        Arc::clone(&state),
        github,
        repos,
        refresh_period,
    ));

    tracing::info!(port, "listening");
    axum::serve(listener, routes::router(state))
        .with_graceful_shutdown(shutdown::signal())
        .await?;
    Ok(())
}
