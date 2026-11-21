use std::{sync::Arc, time::Duration};

use anyhow::Result;
use futures_util::future::join_all;
use tokio::task::block_in_place;

use crate::{
    config::ListingConfig,
    github::{GitHub, Repo},
    listing::Snapshot,
    state::AppState,
};

pub(crate) fn snapshot(config: &ListingConfig, repos: &[Repo]) -> Result<Arc<Snapshot>> {
    let manifests = repos.iter().flat_map(|repo| &repo.manifests);
    block_in_place(|| Snapshot::build(config, manifests)).map(Arc::new)
}

pub(crate) async fn sync(github: &GitHub, repos: &mut [Repo]) -> bool {
    let refreshed = join_all(repos.iter_mut().map(|repo| async move {
        github.refresh(repo).await.unwrap_or_else(|error| {
            tracing::warn!(repo = repo.slug, %error, "refresh failed");
            false
        })
    }))
    .await;
    refreshed.contains(&true)
}

pub(crate) async fn run(
    state: Arc<AppState>,
    github: GitHub,
    mut repos: Vec<Repo>,
    period: Duration,
) {
    loop {
        tokio::time::sleep(period).await;
        if !sync(&github, &mut repos).await {
            continue;
        }
        match snapshot(&state.listing, &repos) {
            Ok(snapshot) => state.snapshot.store(snapshot),
            Err(error) => tracing::error!(%error, "snapshot build failed"),
        }
    }
}
