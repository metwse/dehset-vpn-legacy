#[macro_export]
macro_rules! random_bytes {
    ($n:expr) => {{
        use rand::rand_core::{OsRng, TryRngCore};

        let mut key = [0u8; $n];
        OsRng.try_fill_bytes(&mut key).unwrap();

        key
    }};
}

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
