use std::convert::TryInto;

use hatena_blog_api::Client;
use hatena_blog_api::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response = client.list_categories().await?;
    let categories: Vec<String> = response.try_into()?;
    println!("{:?}", categories);
    Ok(())
}
