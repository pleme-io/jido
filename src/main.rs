use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jido", about = "AI workflow executor")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a workflow from a YAML file
    Run {
        /// Path to the workflow YAML file
        workflow: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { workflow } => {
            println!("jido: executing workflow from {}", workflow.display());
        }
    }
}
