#![allow(dead_code)]
use crate::prelude::*;
use std::time::Duration;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;

enum Command {
    Execute(String, u64),
}

struct CommandRunner {
    sender: UnboundedSender<Command>,
    worker_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl CommandRunner {
    fn new(num_workers: usize) -> Self {
        let (sender, receiver) = unbounded_channel::<Command>();
        let receiver = Arc::new(Mutex::new(receiver));
        let worker_handles = Self::spawn_workers(num_workers, receiver);
        Self {
            sender,
            worker_handles: Arc::new(Mutex::new(worker_handles)),
        }
    }

    fn spawn_workers(
        num_workers: usize,
        receiver: Arc<Mutex<UnboundedReceiver<Command>>>,
    ) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();
        for worker_id in 0..num_workers {
            let receiver = receiver.clone();
            let handle = tokio::spawn(async move {
                loop {
                    let option = {
                        let mut receiver = receiver.lock().await;
                        receiver.recv().await
                    };
                    let Some(Command::Execute(command_id, command_duration)) = option else {
                        println!("Completed shutdown of worker {worker_id}");
                        break;
                    };
                    println!("Starting command {command_id} on worker {worker_id}");
                    sleep(Duration::from_millis(command_duration)).await;
                    println!("Finished command {command_id} on worker {worker_id}");
                }
            });
            handles.push(handle);
        }

        handles
    }

    fn add(&self, command: Command) -> Result<(), SendError<Command>> {
        self.sender.send(command)
    }

    async fn shutdown(self) {
        // Close the sender to signal no more tasks are coming
        drop(self.sender);

        // Wait for all workers to finish
        let handles = {
            let mut guard = self.worker_handles.lock().await;
            take(&mut *guard)
        };

        for handle in handles {
            let _ = handle.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORKER_COUNT: usize = 3;
    const TASK_A_COUNT: usize = 10;
    const TASK_B_COUNT: usize = 2;
    const TASK_A_DURATON: u64 = 200;
    const TASK_B_DURATON: u64 = 200;
    #[allow(clippy::as_conversions, clippy::integer_division)]
    const TASK_A_TOTAL_DURATON: u64 = (TASK_A_COUNT / WORKER_COUNT) as u64 * TASK_A_DURATON;

    #[tokio::test]
    async fn test() {
        // Arrange
        let runner = CommandRunner::new(WORKER_COUNT);

        // Act
        println!("Sending batch A");
        for i in 1..=TASK_A_COUNT {
            let command = Command::Execute(format!("A{i}"), TASK_A_DURATON);
            runner.add(command).expect("should be able to add task");
        }
        println!("Sent batch A");
        sleep(Duration::from_millis(TASK_A_TOTAL_DURATON + 1000)).await;
        println!("Sending batch B");        
        for i in 1..=TASK_B_COUNT {
            let command = Command::Execute(format!("B{i}"), TASK_B_DURATON);
            runner.add(command).expect("should be able to add task");
        }
        println!("Sent batch B");
        println!("Requesting shutdown");
        runner.shutdown().await;
        println!("Completed shutdown");
    }
}
