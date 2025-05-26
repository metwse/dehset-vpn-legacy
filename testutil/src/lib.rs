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
