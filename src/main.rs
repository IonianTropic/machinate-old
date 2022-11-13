use std::io;

mod reader;

fn main() -> io::Result<()> {
    reader::token_rpl()?;
    Ok(())
}
