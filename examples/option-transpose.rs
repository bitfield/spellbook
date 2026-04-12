use std::env;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let port: Option<u16> = env::var("PORT")
        .ok()
        .map(|s| s.parse().context("invalid port"))
        .transpose()?;
    println!("{port:?}");
    Ok(())
}
