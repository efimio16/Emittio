use std::path::Path;
use anyhow::{Result, anyhow};
use emittio_client::Client;

use crate::{AppState, SeedArgs, SeedCmd};

pub const SEED_FILE: &str = "seed.key";

pub fn handle(app: &mut AppState, args: SeedArgs) -> Result<()> {
    match args.command {
        SeedCmd::New => {
            let (client, seed) = Client::new_seed();
            save_to_file(app.dir.join(SEED_FILE), &seed)?;
            app.client = Some(client);
        },
        SeedCmd::Import { file } => {
            let seed = get_from_file(file)?;
            save_to_file(app.dir.join(SEED_FILE), &seed)?;
            app.client = Some(Client::from_seed(seed));
        },
    }

    Ok(())
}

pub fn get_from_file<P: AsRef<Path>>(file: P) -> Result<[u8; 32]> {
    std::fs::read(file)?
        .try_into()
        .map_err(|v: Vec<u8>| {
            anyhow!("invalid seed length: expected 32 bytes, got {}", v.len())
        })
}

pub fn save_to_file<P: AsRef<Path>>(file: P, seed: &[u8; 32]) -> Result<()> {
    Ok(std::fs::write(file, seed)?)
}