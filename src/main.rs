use std::io;

use object::tests::test_obj;

mod reader;
mod object;

fn main() -> io::Result<()> {
    test_obj();
    reader::token_rpl()?;
    Ok(())
}
