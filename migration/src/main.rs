use anyhow::Result;
use sea_orm_migration::prelude::*;
#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var(
        "DATABASE_URL",
        "mysql://root:123456@localhost:3306/ppaass_blog_db",
    );
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
