use std::env;

use anyhow::{Context as _, Result};

fn main() -> Result<()> {
    let port: Option<u16> = env::var("PORT")
        .ok()
        .map(|raw| raw.parse().context("invalid port"))
        .transpose()?;
    println!("{port:?}");
    Ok(())
}
