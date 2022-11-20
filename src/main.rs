use std::io;

mod reader;
mod object;

fn main() -> io::Result<()> {
    reader::token_rpl()?;
    Ok(())
}
