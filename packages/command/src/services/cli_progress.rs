use crate::prelude::*;
use indicatif::ProgressBar;
use tokio::spawn;

pub struct CliProgress<T: ICommandInfo> {
    mediator: Arc<CommandMediator<T>>,
    bar: Arc<ProgressBar>,
    handle: Mutex<Option<JoinHandle<()>>>,
    finished: Arc<Mutex<bool>>,
}

impl<T: ICommandInfo + 'static> CliProgress<T> {
    #[must_use]
    pub fn new(mediator: Arc<CommandMediator<T>>) -> Self {
        Self {
            mediator,
            bar: Arc::new(ProgressBar::new(0)),
            handle: Mutex::default(),
            finished: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn start(&self) {
        let mut handle_guard = self.handle.lock().await;
        if handle_guard.is_some() {
            return;
        }
        let mediator = self.mediator.clone();
        let bar = self.bar.clone();
        let finished = self.finished.clone();
        let handle = spawn(async move {
            let progress = mediator.get_progress().await;
            update(&bar, &progress);
            while !*finished.lock().await {
                let progress = mediator.wait_for_progress().await;
                update(&bar, &progress);
            }
        });
        *handle_guard = Some(handle);
    }

    pub async fn finish(&self) {
        let mut finished_guard = self.finished.lock().await;
        *finished_guard = true;
        drop(finished_guard);
        let mut handle_guard = self.handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
        drop(handle_guard);
        self.bar.finish();
    }
}

#[allow(clippy::as_conversions)]
fn update(bar: &ProgressBar, progress: &CommandProgress) {
    bar.set_length(progress.total as u64);
    bar.set_position(progress.completed as u64);
}

impl<T: ICommandInfo + 'static> Service for CliProgress<T> {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(services.get_service().await?))
    }
}
