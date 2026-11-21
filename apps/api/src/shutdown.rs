use std::future;

use tokio::signal::{
    ctrl_c,
    unix::{self, SignalKind},
};

pub(crate) async fn signal() {
    tokio::select! {
        () = terminate() => {}
        _ = ctrl_c() => {}
    }
}

async fn terminate() {
    match unix::signal(SignalKind::terminate()) {
        Ok(mut stream) => {
            stream.recv().await;
        }
        Err(error) => {
            tracing::error!(%error, "failed to install the SIGTERM handler");
            future::pending::<()>().await;
        }
    }
}
