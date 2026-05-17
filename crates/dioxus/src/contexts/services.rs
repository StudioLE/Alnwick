use crate::prelude::*;
use tokio::sync::OnceCell;
use tokio::sync::broadcast::Receiver;

const WORKERS: usize = 4;

static SERVICES: OnceCell<Arc<ServiceProvider>> = OnceCell::const_new();
static RUNNER: OnceCell<Arc<CommandRunner<CommandInfo>>> = OnceCell::const_new();
static MEDIATOR: OnceCell<Arc<CommandMediator<CommandInfo>>> = OnceCell::const_new();
static METADATA: OnceCell<Arc<MetadataRepository>> = OnceCell::const_new();
static ADD_HANDLER: OnceCell<Arc<AddHandler>> = OnceCell::const_new();

/// Build, initialize, and install the shared [`ServiceProvider`].
///
/// - Registers core and command services
/// - Installs the global tracing subscriber via [`ServiceProvider::init`]
/// - Stores the container so every later resolution uses the same instance
/// - Panics if called more than once
pub fn init_server() {
    let services = ServiceBuilder::new()
        .with_core()
        .with_commands()
        .build()
        .expect_init();
    let services = Arc::new(services);
    assert!(
        SERVICES.set(services).is_ok(),
        "services should not already be installed"
    );
}

async fn init_runner() -> Arc<CommandRunner<CommandInfo>> {
    let runner = get_services()
        .expect_async::<CommandRunner<CommandInfo>>()
        .await;
    runner.start(WORKERS).await;
    runner
}

async fn init_mediator() -> Arc<CommandMediator<CommandInfo>> {
    get_services()
        .expect_async::<CommandMediator<CommandInfo>>()
        .await
}

async fn init_metadata() -> Arc<MetadataRepository> {
    get_services().expect_async::<MetadataRepository>().await
}

fn get_services() -> &'static Arc<ServiceProvider> {
    SERVICES
        .get()
        .expect("services should be installed via entry::start")
}

pub async fn get_runner() -> &'static Arc<CommandRunner<CommandInfo>> {
    RUNNER.get_or_init(init_runner).await
}

async fn get_mediator() -> &'static Arc<CommandMediator<CommandInfo>> {
    MEDIATOR.get_or_init(init_mediator).await
}

pub async fn subscribe_to_events() -> Receiver<CommandEvent> {
    get_mediator().await.subscribe()
}

pub async fn get_metadata() -> &'static Arc<MetadataRepository> {
    METADATA.get_or_init(init_metadata).await
}

async fn init_add_handler() -> Arc<AddHandler> {
    get_services().expect_async::<AddHandler>().await
}

pub async fn get_add_handler() -> &'static Arc<AddHandler> {
    ADD_HANDLER.get_or_init(init_add_handler).await
}
