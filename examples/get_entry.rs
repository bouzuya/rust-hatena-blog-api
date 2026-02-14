use std::convert::TryInto;
use std::env;

use anyhow::Context as _;
use hatena_blog_api::Client;
use hatena_blog_api::Config;
use hatena_blog_api::Entry;
use hatena_blog_api::EntryId;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entry_id = env::args().nth(1).context("no args")?.parse::<EntryId>()?;
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response = client.get_entry(&entry_id).await?;
    let entry: Entry = response.try_into()?;
    println!("{:?}", entry);
    Ok(())
}
