use forgequeue::{config::Config, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;
    run(config).await?;
    Ok(())
}
