use crate::{Client, Error};
use proto_core::{Token, cmd};
use tokio::io::SimplexStream;

impl Client {
    pub async fn sign_token(_token: Token) -> cmd::SignTokenResponse {
        todo!()
    }

    pub async fn revoke_token(_token_id: u64) -> cmd::RevokeTokenResponse {
        todo!()
    }

    pub async fn new_connection(_token_id: u64, _port: u16) -> Result<SimplexStream, Error> {
        todo!()
    }
}
