#![allow(dead_code)]

use crate::prelude::*;
use tokio::sync::MutexGuard;

/// A mediator between the [`CommandRunner`], [`Worker`] and [`CliProgress`] services.
pub struct CommandMediator<T: ICommandInfo> {
    /// Number of commands queued over time
    progress: Mutex<CommandProgress>,
    /// Queue of commands to execute
    queue: Mutex<VecDeque<T::Command>>,
    /// Notify workers when new work is available
    notify_workers: Notify,
    /// Notify progress subscribers when work is queued, executing, or completed
    notify_progress: Notify,
    /// Current status of the runner
    ///
    /// `RwLock` allows multiple readers
    status: RwLock<RunnerStatus>,
    /// Current status of the runner
    ///
    /// `RwLock` allows multiple readers
    results: Mutex<Vec<T::Result>>,
}

impl<T: ICommandInfo + 'static> Service for CommandMediator<T> {
    type Error = Infallible;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new())
    }
}

impl<T: ICommandInfo> CommandMediator<T> {
    pub(super) fn new() -> Self {
        Self {
            progress: Mutex::default(),
            queue: Mutex::default(),
            notify_workers: Notify::default(),
            notify_progress: Notify::default(),
            status: RwLock::default(),
            results: Mutex::default(),
        }
    }

    async fn get_status(&self) -> RunnerStatus {
        *self.status.read().await
    }

    async fn update_progress(&self, callback: fn(&mut MutexGuard<CommandProgress>)) {
        let mut progress = self.progress.lock().await;
        callback(&mut progress);
        drop(progress);
        self.notify_progress.notify_waiters();
    }
}

// Implementation for `CommandRunner`
impl<T: ICommandInfo> CommandMediator<T> {
    pub(super) async fn set_status(&self, status: RunnerStatus) {
        let mut status_guard = self.status.write().await;
        *status_guard = status;
        drop(status_guard);
        self.notify_workers.notify_waiters();
    }

    /// Add a command to the queue and notify a worker and notify progress subscribers.
    pub(super) async fn queue_command(&self, command: T::Command) {
        let mut queue = self.queue.lock().await;
        queue.push_back(command);
        drop(queue);
        self.notify_workers.notify_one();
        self.update_progress(|progress| {
            progress.queued += 1;
            progress.total += 1;
        })
        .await;
    }

    /// Get the results.
    ///
    /// Note: The [`MutexGuard`] must be dropped or the [`Worker`] will be unable to finish
    /// execution.
    pub(super) async fn get_results(&self) -> MutexGuard<'_, Vec<T::Result>> {
        self.results.lock().await
    }
}

// Implementation for `Worker`
impl<T: ICommandInfo> CommandMediator<T> {
    /// Get the next instruction.
    pub(super) async fn get_instruction(&self) -> Instruction<'_, T> {
        let mut queue_guard = self.queue.lock().await;
        if self.get_status().await == RunnerStatus::Stopping {
            return Instruction::Stop;
        }
        if let Some(command) = queue_guard.pop_front() {
            drop(queue_guard);
            self.update_progress(|progress| {
                progress.queued -= 1;
                progress.executing += 1;
            })
            .await;
            return Instruction::Execute(command);
        }
        drop(queue_guard);
        if self.get_status().await == RunnerStatus::Draining {
            return Instruction::Stop;
        }
        Instruction::Wait(self.notify_workers.notified())
    }

    /// Add a result.
    pub(super) async fn add_result(&self, result: T::Result) {
        let mut results = self.results.lock().await;
        results.push(result);
        drop(results);
        self.update_progress(|progress| {
            progress.executing -= 1;
            progress.completed += 1;
        })
        .await;
    }
}

// Implementation for `Progress` subscribers
impl<T: ICommandInfo> CommandMediator<T> {
    /// Get the current progress.
    pub async fn get_progress(&self) -> CommandProgress {
        let guard = self.progress.lock().await;
        (*guard).clone()
    }

    /// Wait for progress to be reported
    pub async fn wait_for_progress(&self) -> CommandProgress {
        self.notify_progress.notified().await;
        self.get_progress().await
    }
}
