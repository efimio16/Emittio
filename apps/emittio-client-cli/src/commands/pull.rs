use anyhow::{Context, Result};
use emittio_crypto::id::Id;
use emittio_inbox::Message;

use crate::{AppState, PullArgs};

struct MessagePrinter<'a>(&'a Id, &'a Message);

impl<'a> MessagePrinter<'a> {
    fn display(&self) -> String {
        format!("{}@emittio | {} | Message {}", Id::hash_from(&self.1.from).expect("failed to hash object"), self.1.subject, self.0)
    }
}

pub async fn handle(app: &mut AppState, args: PullArgs) -> Result<()> {
    let client = app.client.as_mut().context("client not initialized")?;

    let inbox_name = args.inbox;
    let inbox = client.use_inbox(&inbox_name);

    let res = inbox.pull().await?;

    if res.len() == 0 {
        println!("No new messages.");
    } else {
        for message_entry in res {
            println!("{}", MessagePrinter(&message_entry.0, &message_entry.1).display());
        }
    }

    Ok(())
}