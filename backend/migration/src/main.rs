use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv()?;
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
