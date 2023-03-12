
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::str::FromStr;

fn main() -> Result<()>{
    let chunkType = chunk_type::ChunkType::from_str("Ru1t")?;
    // let chunkType = chunk_type::ChunkType::try_from([75, 120,120,120])?;
    print!("{}", chunkType);
    Ok(())
    // todo!();
}
