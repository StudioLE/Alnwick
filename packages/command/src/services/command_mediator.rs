#![allow(dead_code)]
use crate::prelude::*;

/// A mediator between the command queue and the workers.
pub struct WorkerMediator<T: ICommandInfo> {
    /// Number of commands queued over time
    pub(super) all_time_queued: Mutex<usize>,
    /// Number of commands completed over time
    pub(super) all_time_completed: Mutex<usize>,
    /// Queue of commands to execute
    pub(super) queue: Mutex<VecDeque<T::Command>>,
    /// Notify workers when new work is available
    pub(super) notify_workers: Notify,
    /// Notify progress subscribers when work is queued, executing, or completed
    pub(super) notify_progress: Notify,
    /// Current status of the runner
    ///
    /// `RwLock` allows multiple readers
    pub(super) status: RwLock<RunnerStatus>,
    /// Current status of the runner
    ///
    /// `RwLock` allows multiple readers
    pub(super) results: Mutex<Vec<T::Result>>,
}

impl<T: ICommandInfo + 'static> Service for WorkerMediator<T> {
    type Error = Infallible;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new())
    }
}

impl<T: ICommandInfo> WorkerMediator<T> {
    pub(super) fn new() -> Self {
        Self {
            all_time_queued: Mutex::default(),
            all_time_completed: Mutex::default(),
            queue: Mutex::default(),
            notify_workers: Notify::default(),
            notify_progress: Notify::default(),
            status: RwLock::default(),
            results: Mutex::default(),
        }
    }

    pub(super) async fn get_instruction(&self) -> Instruction<'_, T> {
        let mut queue_guard = self.queue.lock().await;
        if self.get_status().await == RunnerStatus::Stopping {
            return Instruction::Stop;
        }
        if let Some(command) = queue_guard.pop_front() {
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
        let mut completed = self.all_time_completed.lock().await;
        *completed += 1;
        drop(completed);
        self.notify_progress.notify_waiters();
    }

    pub(super) async fn get_status(&self) -> RunnerStatus {
        *self.status.read().await
    }

    pub(super) async fn set_status(&self, status: RunnerStatus) {
        let mut status_guard = self.status.write().await;
        *status_guard = status;
        drop(status_guard);
        self.notify_workers.notify_waiters();
    }
}
