use std::u64;

use proto_core::{Token, TokenScope, TokenTag};

pub fn generate_token(id: u64, name: String, tags: Vec<String>) -> Token {
    Token {
        sub: id,
        iat: 0,
        exp: u64::MAX,
        level: u64::MAX,
        name,
        tags,
        scope: vec![
            TokenScope::ForwardPort,
            TokenScope::RequestPort {
                tags: vec![TokenTag::Regex(String::from("*"))],
                ports: vec![0..=u16::MAX],
            },
        ],
    }
}

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! send_handshake_payload {
    ($w: expr, $content_type:expr, $payload:expr) => {
        {
            let payload = bincode::serde::encode_to_vec(&$payload, bincode::config::standard())?;

            write_handshake_payload($w, $content_type, &payload).await.unwrap();
        }
    };
}
