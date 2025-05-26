use server::ServerBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let server = ServerBuilder {
        addr: "0.0.0.0:3781".parse()?,
        encryption_key: vec![0; 16],
        signing_key: vec![0; 32],
    }
    .try_build()
    .await?;

    server.serve().await?;
    Ok(())
}
