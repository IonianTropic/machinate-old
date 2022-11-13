use std::io::{self, BufRead, Write};

use self::{scanner::Scanner, token::Token};

mod scanner;
mod token;

fn token_read<B: BufRead>(input: &mut B) -> Vec<Token> {
    let mut scanner = Scanner::new();
    scanner.scan(input);
    
    scanner.token_stream
}

fn token_print<W: Write>(output: &mut W, token_stream: Vec<Token>) -> io::Result<()> {
    let setup = format!("{:?}\n", token_stream);
    let buf = setup.as_bytes();
    output.write(buf)?;
    output.flush()?;
    Ok(())
}

pub fn token_rpl() -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let prompt = ">>> ".as_bytes();
    loop {
        stdout.write(prompt)?;
        stdout.flush()?;
        token_print(&mut stdout, token_read(&mut stdin))?;
    }
}
