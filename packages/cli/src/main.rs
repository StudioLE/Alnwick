//! Binary entrypoint for `alnwick`.

use alnwick_core::prelude::*;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    Cli::new().run().await
}
