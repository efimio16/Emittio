use std::io::Read;

use anyhow::{Context, Result};
use tokio::io::AsyncRead;

use crate::{AppState, SendArgs};

pub async fn handle(app: &mut AppState, args: SendArgs) -> Result<()> {
    let client = app.client.as_mut().context("client not initialized")?;

    let inbox_name = args.from;
    let inbox = client.use_inbox(&inbox_name);

    let recipient_address = postcard::from_bytes(&std::fs::read(args.to)?)?;

    let body: Box<dyn AsyncRead + Send + Unpin> = if args.edit {
        let body_string = edit::edit("\n\nSent with Emittio CLI.")?;
        Box::new(std::io::Cursor::new(body_string))
    } else if let Some(body_file) = args.body_file {
        Box::new(tokio::fs::File::from_std(std::fs::File::open(body_file)?))
    } else if let Some(body_string) = args.body {
        Box::new(std::io::Cursor::new(body_string))
    } else {
        let mut body_string = String::new();

        std::io::stdin().read_to_string(&mut body_string)?;
        Box::new(std::io::Cursor::new(body_string))
    };

    inbox.send(args.subject, recipient_address, body).await?;

    println!("Message send.");

    Ok(())
}