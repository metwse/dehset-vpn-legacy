use client::ClientBuilder;
use crypto::sign::{Hs256, sign_token};
use testutil::generate_token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let token = generate_token(1, String::from("test"), vec![]);
    let signed_token = sign_token(token, &Hs256::try_new(&[0; 16])?)?;

    let _client = ClientBuilder {
        addr: "127.0.0.1:3781".parse()?,
        encryption_key: vec![0; 16],
        token: signed_token,
    }
    .try_build()
    .await?;

    Ok(())
}
