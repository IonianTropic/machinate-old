use std::io;

use object::tests::test_obj;

mod reader;
mod object;

fn main() -> io::Result<()> {
    reader::token_rpl()?;
    test_obj();
    Ok(())
}
