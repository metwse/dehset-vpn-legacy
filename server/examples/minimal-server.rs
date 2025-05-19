use server::ServerBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = ServerBuilder {
        addr: "0.0.0.0:3781".parse().unwrap(),
        encryption_key: vec![0; 16],
        signing_key: vec![0; 32],
    }
    .try_build()
    .await?;

    server.serve().await?;
    Ok(())
}
