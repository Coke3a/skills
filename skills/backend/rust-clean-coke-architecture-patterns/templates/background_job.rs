// Optional architecture variant template. Background job workflow is out of
// scope for this skill unless the user specifically needs to preserve the same
// Clean Architecture dependency direction for a periodic task.
//
// Keep the direction:
// handler/spawner -> usecase -> domain repository trait
// infra repository -> domain repository trait

use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::usecases::RunExampleMaintenanceUseCase;

pub fn spawn_example_maintenance(
    usecase: Arc<RunExampleMaintenanceUseCase>,
    cancel: CancellationToken,
    interval_secs: u64,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        info!("example maintenance task started");

        loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    info!("example maintenance task shutting down");
                    break;
                }
                _ = interval.tick() => {
                    if let Err(err) = usecase.execute().await {
                        error!(error = %err, "example maintenance task failed");
                    }
                }
            }
        }
    })
}
