use std::str::FromStr;

use crate::chunk_type::ChunkType;

// mod args;
// mod chunk;
mod chunk_type;
// mod commands;
// mod png;

// bytes are u8
// String -> Vec<u8>

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chunk = ChunkType::from_str("Rust").unwrap();
    // todo!()
    Ok(())
}
