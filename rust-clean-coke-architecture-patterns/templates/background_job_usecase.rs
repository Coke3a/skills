// ============================================
// Usecase: src/usecases/background/heartbeat_sweeper.rs
// ============================================
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tracing::{error, warn};

use crate::domain::repositories::ForwardingSessionRepository;
use crate::usecases::UsecaseError;

pub struct HeartbeatSweeperUseCase {
    session_repo: Arc<dyn ForwardingSessionRepository>,
}

impl HeartbeatSweeperUseCase {
    pub fn new(session_repo: Arc<dyn ForwardingSessionRepository>) -> Self {
        Self { session_repo }
    }

    pub async fn sweep_stale_sessions(&self, timeout: Duration, limit: i64) -> Result<usize, UsecaseError> {
        let cutoff = Utc::now() - timeout;
        let stale = self.session_repo.find_stale_sessions(&cutoff, limit).await?;

        let mut count = 0;
        for mut session in stale {
            if let Err(e) = session.mark_disconnected() {
                warn!(
                    session_id = %session.id(),
                    status = %session.status(),
                    error = %e,
                    "Failed to mark stale session as disconnected (state transition error)"
                );
                continue;
            }

            match self.session_repo.update_if_active(&session).await {
                Ok(updated) => {
                    if updated { count += 1; }
                }
                Err(e) => {
                    error!(session_id = %session.id(), error = %e, "Failed to persist disconnected session");
                    continue;
                }
            }
        }

        Ok(count)
    }
}

// ============================================
// Handler spawner: src/handlers/heartbeat/mod.rs
// ============================================
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::usecases::HeartbeatSweeperUseCase;

pub fn spawn(
    usecase: Arc<HeartbeatSweeperUseCase>,
    cancel: CancellationToken,
    sweep_interval_secs: u64,
    timeout_secs: u64,
    batch_limit: i64,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!("Heartbeat sweeper started");

        let mut interval = tokio::time::interval(Duration::from_secs(sweep_interval_secs));

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    info!("Heartbeat sweeper shutting down");
                    break;
                }
                _ = interval.tick() => {
                    match usecase.sweep_stale_sessions(Duration::from_secs(timeout_secs), batch_limit).await {
                        Ok(count) if count > 0 => {
                            info!(count, "Disconnected stale sessions");
                        }
                        Ok(_) => {}
                        Err(e) => {
                            error!(error = %e, "Heartbeat sweep failed");
                        }
                    }
                }
            }
        }
    })
}
