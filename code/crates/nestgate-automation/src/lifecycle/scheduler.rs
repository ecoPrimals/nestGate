// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Background evaluation interval and task dispatch loop.

use tokio::sync::mpsc;
use tracing::debug;

use crate::Result;

use super::DatasetLifecycleManager;
use super::ScheduledTask;

impl DatasetLifecycleManager {
    pub(super) async fn start_scheduler(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<ScheduledTask>(100);
        *self.scheduler.write().await = Some(tx.clone());

        let scheduler_tx = tx;
        let evaluation_interval = self.config.evaluation_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(evaluation_interval);
            loop {
                interval.tick().await;
                if scheduler_tx
                    .send(ScheduledTask::PolicyUpdate)
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                match task {
                    ScheduledTask::EvaluateDataset(name) => {
                        debug!(dataset = name.as_str(), "Scheduled evaluation");
                    }
                    ScheduledTask::ExecuteAction(name, action) => {
                        debug!(dataset = name.as_str(), action = ?action, "Scheduled action");
                    }
                    ScheduledTask::PolicyUpdate => {
                        debug!("Scheduled policy update");
                    }
                    ScheduledTask::StatsCollection => {
                        debug!("Scheduled stats collection");
                    }
                }
            }
        });
        Ok(())
    }
}
