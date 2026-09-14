use crate::{repositories::postgres, state::AppState};

pub fn spawn(state: AppState) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(state.config.recovery_interval);
        loop {
            ticker.tick().await;
            match postgres::recover(&state.pool).await {
                Ok(n) if n > 0 => tracing::warn!(count=n, "recovered expired leases"),
                Err(error) => tracing::error!(?error, "lease recovery failed"),
                _ => {}
            }
        }
    });
}
