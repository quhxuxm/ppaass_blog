use sea_orm_migration::prelude::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
