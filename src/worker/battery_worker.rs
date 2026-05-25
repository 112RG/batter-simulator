use std::time::Duration;

use chrono::Utc;
use tokio::time::{MissedTickBehavior, interval};
use tokio_util::sync::CancellationToken;
use tracing::info;

pub async fn battery_worker(shutdown: CancellationToken) -> anyhow::Result<()> {
    let mut tick = interval(Duration::from_secs(1));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut flush_tick = interval(Duration::from_secs(10));
    flush_tick.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                break;
            }

            _ = tick.tick() => {
                let now = Utc::now();
                info!("Battery worker tick at {now}");
            }

            _ = flush_tick.tick() => {
                info!("Flushing battery readings to database");
            }
        }
    }

    Ok(())
}
