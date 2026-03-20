use crate::prelude::*;
use tokio::sync::OnceCell;
use tokio::sync::broadcast::Receiver;

const WORKERS: usize = 4;

static SERVICES: OnceCell<Arc<ServiceProvider>> = OnceCell::const_new();
static RUNNER: OnceCell<Arc<CommandRunner<CommandInfo>>> = OnceCell::const_new();
static MEDIATOR: OnceCell<Arc<CommandMediator<CommandInfo>>> = OnceCell::const_new();
static METADATA: OnceCell<Arc<MetadataRepository>> = OnceCell::const_new();
static ADD_HANDLER: OnceCell<Arc<AddHandler>> = OnceCell::const_new();

async fn init_services() -> Arc<ServiceProvider> {
    Arc::new(
        ServiceProvider::new()
            .with_commands()
            .await
            .expect("should be able to create provider with commands"),
    )
}

async fn init_runner() -> Arc<CommandRunner<CommandInfo>> {
    let services = get_services().await;
    let runner = services
        .get_service::<CommandRunner<CommandInfo>>()
        .await
        .expect("should be able to get command runner");
    runner.start(WORKERS).await;
    runner
}

async fn init_mediator() -> Arc<CommandMediator<CommandInfo>> {
    let services = get_services().await;
    services
        .get_service::<CommandMediator<CommandInfo>>()
        .await
        .expect("should be able to get command mediator")
}

async fn init_metadata() -> Arc<MetadataRepository> {
    let services = get_services().await;
    services
        .get_service::<MetadataRepository>()
        .await
        .expect("should be able to get metadata repository")
}

async fn get_services() -> &'static Arc<ServiceProvider> {
    SERVICES.get_or_init(init_services).await
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
    let services = get_services().await;
    services
        .get_service::<AddHandler>()
        .await
        .expect("should be able to get add handler")
}

pub async fn get_add_handler() -> &'static Arc<AddHandler> {
    ADD_HANDLER.get_or_init(init_add_handler).await
}
