
#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Nil,
    Dot,
    Char(char),
    Number(NumType),
    Symbol(String),
}

#[derive(Debug)]
pub enum NumType {
    Int(i32),
    Float(f32),
    // Complex(f32, f32)
}
