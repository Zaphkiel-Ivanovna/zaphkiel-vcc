use arc_swap::ArcSwap;

use crate::{config::ListingConfig, listing::Snapshot};

pub(crate) struct AppState {
    pub(crate) listing: ListingConfig,
    pub(crate) snapshot: ArcSwap<Snapshot>,
}
