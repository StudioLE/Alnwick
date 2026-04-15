use alnwick_core::prelude::*;
use std::process::exit;

#[tokio::main]
async fn main() {
    init_logger().expect("should be able to init logger");
    let cli = Cli::parse();
    let services = ServiceBuilder::new().with_core().with_commands().build();
    match cli.command {
        Command::Add(options) => {
            let command = services
                .get_async::<AddCliCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to add podcast");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Fetch(options) => {
            let command = services
                .get_async::<FetchCliCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to fetch podcast");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Download(options) => {
            let command = services
                .get_async::<DownloadCliCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to download podcast");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Emulate(options) => {
            let command = services
                .get_async::<EmulateCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to create RSS feeds");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Cover(options) => {
            let command = services
                .get_async::<CoverCommand>()
                .await
                .expect("should be able to get command");
            if let Err(e) = command.execute(options).await {
                error!("Failed to create banner and cover images");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
    }
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Add a new podcast from an RSS feed.
    Add(AddOptions),
    /// Fetch an existing podcast using its stored feed URL.
    Fetch(FetchOptions),
    /// Download episodes of a podcast.
    Download(DownloadOptions),
    /// Create emulated RSS of a podcast.
    Emulate(EmulateOptions),
    /// Download cover and banner images of a podcast.
    Cover(CoverOptions),
}
