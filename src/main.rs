
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()>{
    let chunk_type = chunk_type::ChunkType::try_from([64, 55,55,129])?;
    chunk_type.print(3);
    Ok(())
    // todo!();
}
