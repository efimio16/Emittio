mod commands;

use std::path::PathBuf;
use anyhow::Context;
use clap::{Parser, Subcommand};
use emittio_client::Client;
use directories::ProjectDirs;

use crate::commands::{pull, seed::{self, SEED_FILE}, send};

const APP_NAME: &str = "emittio";

#[derive(Parser)]
#[command(name = APP_NAME, version, about = "Private, decentralized mail app")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Seed(SeedArgs),
    Send(SendArgs),
    Pull(PullArgs),
}

#[derive(Parser)]
struct SeedArgs {
    #[command(subcommand)]
    command: SeedCmd,
}

#[derive(Subcommand)]
enum SeedCmd {
    New,
    Import {
        #[arg(long)]
        file: PathBuf,
    },
}

#[derive(Parser)]
struct SendArgs {
    #[arg(long)]
    from: String,

    #[arg(long)]
    to: PathBuf,

    #[arg(long)]
    subject: String,

    #[arg(long, conflicts_with_all = ["body_file", "edit"])]
    body: Option<String>,

    #[arg(long, conflicts_with_all = ["body", "edit"])]
    body_file: Option<PathBuf>,

    #[arg(long, conflicts_with_all = ["body", "body_file"])]
    edit: bool,
}

#[derive(Parser)]
struct PullArgs {
    #[arg(long)]
    inbox: String,
}

struct AppState {
    client: Option<Client>,
    dir: PathBuf,
}

impl AppState {
    pub fn new(client: Option<Client>, dir: PathBuf) -> Self {
        Self { client, dir }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let proj = ProjectDirs::from("", "", APP_NAME).context("failed to determine app dirs")?;
    let app_dir = proj.data_dir().to_path_buf();

    let client = if std::fs::exists(app_dir.join(SEED_FILE))? {
        Some(Client::from_seed(seed::get_from_file(app_dir.join(SEED_FILE))?))
    } else {
        None
    };

    let mut app = AppState::new(client, app_dir);

    match cli.command {
        Command::Seed(args) => seed::handle(&mut app, args)?,
        Command::Send(args) => send::handle(&mut app, args).await?,
        Command::Pull(args) => pull::handle(&mut app, args).await?,
    }

    Ok(())
}