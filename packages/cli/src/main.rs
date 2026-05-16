use alnwick_core::prelude::*;
use std::process::exit;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let services = ServiceBuilder::new()
        .with_core()
        .with_commands()
        .build()
        .expect_init();
    match cli.command {
        Command::Add(options) => {
            let command = services.expect_async::<AddCliCommand>().await;
            if let Err(e) = command.execute(options).await {
                error!("Failed to add podcast");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Fetch(options) => {
            let command = services.expect_async::<FetchCliCommand>().await;
            if let Err(e) = command.execute(options).await {
                error!("Failed to fetch podcast");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Download(options) => {
            let command = services.expect_async::<DownloadCliCommand>().await;
            if let Err(e) = command.execute(options).await {
                error!("Failed to download podcast");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Emulate(options) => {
            let command = services.expect_async::<EmulateCliCommand>().await;
            if let Err(e) = command.execute(options).await {
                error!("Failed to create RSS feeds");
                eprintln!("{}", e.render());
                exit(1);
            }
        }
        Command::Cover(options) => {
            let command = services.expect_async::<CoverCliCommand>().await;
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
    Fetch(PodcastOptions),
    /// Download episodes of a podcast.
    Download(DownloadOptions),
    /// Create emulated RSS of a podcast.
    Emulate(PodcastOptions),
    /// Download cover and banner images of a podcast.
    Cover(PodcastOptions),
}
