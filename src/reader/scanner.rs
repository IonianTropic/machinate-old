use std::io::{BufRead, Write};
use std::process::exit;
use std::str::FromStr;

use super::token::{Token, NumType};

const VERBOSE_DEBUG: bool = true;

/// Handwritten scanner without regular expressions
#[derive(Debug)]
pub struct Scanner {
    state: ScannerState,
    pub token_stream: Vec<Token>,
    next_string: String,
    quote_string: String,
    paren_count: i32,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            state: ScannerState::new(),
            token_stream: Vec::new(),
            next_string: String::new(),
            quote_string: String::new(),
            paren_count: 0,
        }
    }

    pub fn scan<B: BufRead>(&mut self, input: &mut B) {
        let mut buf = String::new();
        'line: loop {
            match input.read_line(&mut buf) {
                Ok(len) => if len > 80 { panic!("Maximum line length (80) reached"); }
                Err(e) => panic!("read_line error: {}", e),
            }
            if buf.trim() == "quit" {exit(0)}
            if VERBOSE_DEBUG {println!("Buffer Debug: {:?}", buf);}
            for ch in buf.chars() {
                if VERBOSE_DEBUG {
                    self.debug();
                    println!("ch Debug: {:?}", ch);
                }
                match self.state {
                    // Main control flow
                    ScannerState::Start => self.full_start(ch),
                    ScannerState::Error(e) => {
                        panic!("Scanner Error: {}", e)
                    }
                    ScannerState::Comment => continue 'line, // skip to next line
                    ScannerState::LParen => {
                        if ch == ')' {
                            if self.paren_count == 0 {
                                self.state.set_error(0);
                            }
                            self.paren_count -= 1;
                            self.token_stream.push(Token::Nil);
                            self.state.set_start();
                        } else {
                            self.token_stream.push(Token::LParen);
                            self.full_start(ch);
                        }
                    }
                    ScannerState::RParen => {
                        self.token_stream.push(Token::RParen);
                        self.full_start(ch);
                    }
                    // Intermediate scanning
                    ScannerState::SignOrSymbol => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                                self.state.set_first_digits();
                            }
                            'N' | 'n' => {
                                self.next_string.push(ch);
                                self.state.set_nan_or_symbol(0);
                            }
                            'I' | 'i' => {
                                self.next_string.push(ch);
                                self.state.set_inf_or_symbol(0);
                            }
                            '.' => {
                                self.next_string.push(ch);
                                self.state.set_dot_or_digits();
                            }
                            ch if is_symbol_continue(ch) => {
                                self.next_string.push(ch);
                                self.state.set_symbol();
                            }
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::Symbol(
                                        self.next_string.clone()
                                    ));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::NaNOrSymbol(step) => {
                        match ch {
                            ch if is_nan(ch, step) => {
                                self.next_string.push(ch);
                                self.state.set_nan_or_symbol(step+1);
                            }
                            '+' | '-' => {
                                if step == 2 {
                                    todo!("Complex numbers not yet implemented");
                                } else {
                                    self.next_string.push(ch);
                                    self.state.set_symbol();
                                }
                            }
                            ch if is_symbol_continue(ch) => {
                                self.next_string.push(ch);
                                self.state.set_symbol();
                            }
                            ch if self.start(ch) => {
                                if step == 2 {
                                    self.token_stream.push(Token::Number(
                                        NumType::Float(
                                            f32::from_str(&self.next_string)
                                            .unwrap()
                                        )));
                                    self.next_string.clear();
                                } else {
                                    self.token_stream.push(Token::Symbol(self.next_string.clone()));
                                    self.next_string.clear();
                                }
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::InfOrSymbol(step) => {
                        match ch {
                            ch if is_inf(ch, step) => {
                                self.next_string.push(ch);
                                self.state.set_inf_or_symbol(step+1);
                            }
                            '+' | '-' => {
                                if step == 2 || step == 7 {
                                    todo!("Complex numbers not yet implemented");
                                } else {
                                    self.next_string.push(ch);
                                    self.state.set_symbol();
                                }
                            }
                            ch if is_symbol_continue(ch) => {
                                self.next_string.push(ch);
                                self.state.set_symbol();
                            }
                            ch if self.start(ch) => {
                                if step == 2 || step == 7 {
                                    self.token_stream.push(Token::Number(
                                        NumType::Float(
                                            f32::from_str(&self.next_string)
                                            .unwrap()
                                        )));
                                    self.next_string.clear();
                                } else {
                                    self.token_stream.push(Token::Symbol(self.next_string.clone()));
                                    self.next_string.clear();
                                }
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::DotOrDigits => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                                self.state.set_2nd_digits();
                            }
                            ch if self.start(ch) => self.token_stream.push(Token::Dot),
                            _ => self.state.set_error(0),
                        }
                    }
                    // char scanning
                    // ScannerState::CharStart => {
                    //     match ch {
                    //         '\'' => self.state.set_error(0),
                    //         '\\' => self.state.set_char_escape(),
                    //         '\x20'..= '\u{d7ff}' |
                    //         '\u{e000}'..= '\u{10ffff}' => {
                    //             self.next_string.push(ch);
                    //             self.state.set_char_point();
                    //         }
                    //         _ => self.state.set_error(0),
                    //     }
                    // }
                    // ScannerState::CharPoint => {
                    //     match ch {
                    //         '\'' => {
                    //             self.token_stream.push(
                    //                 Token::Char(
                    //                     self.next_string
                    //                     .chars()
                    //                     .next()
                    //                     .unwrap()
                    //                 ));
                    //             self.next_string.clear();
                    //         }
                    //         ch if is_symbol_start(ch) => {
                    //             self.state.set_symbol();
                    //         }
                    //         _ => self.state.set_error(0),
                    //     }
                    // }
                    // ScannerState::CharEscape => {
                    //     // TODO implement escape sequences u, x
                    //     match ch {
                    //         '0' => {
                    //             self.next_string.push('\0');
                    //             self.state.set_char_point();
                    //         }
                    //         't' => {
                    //             self.next_string.push('\t');
                    //             self.state.set_char_point();
                    //         }
                    //         'n' => {
                    //             self.next_string.push('\n');
                    //             self.state.set_char_point();
                    //         }
                    //         'r' => {
                    //             self.next_string.push('\r');
                    //             self.state.set_char_point();
                    //         }
                    //         '\\' => {
                    //             self.next_string.push('\\');
                    //             self.state.set_char_point();
                    //         }
                    //         '\'' => {
                    //             self.next_string.push('\'');
                    //             self.state.set_char_point();
                    //         }
                    //         _ => self.state.set_error(0),
                    //     }
                    // }
                    // number scanning
                    ScannerState::CharOrQuote => {
                        match ch {
                            '\\' => {
                                self.quote_string.push(ch);
                                self.state.set_char_escape_or_quote();
                            }
                            ch if is_symbol_start(ch) => {
                                self.next_string.push(ch);
                                self.quote_string.push(ch);
                                self.state.set_char_point_or_quote();
                            }
                            '\0' ..= '\u{D7FF}' |
                            '\u{E000}' ..= '\u{10FFFF}' => {
                                self.next_string.push(ch);
                                self.state.set_char_point();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::CharPoint => {
                        match ch {
                            '\'' => {
                                self.token_stream.push(
                                    Token::Char(
                                        self.next_string
                                        .chars()
                                        .next()
                                        .unwrap()
                                    ));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::CharPointOrQuote => {
                        match ch {
                            '\'' => {
                                self.state.set_char_end();
                            }
                            ch if is_symbol_continue(ch) => {
                                self.quote_string.push(ch);
                                self.next_string.clear();
                                self.state.set_quote();
                            }
                            ch if self.start(ch) => {
                                self.token_stream.push(Token::Quote(self.quote_string.clone()));
                                self.quote_string.clear();
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::CharEscapeOrQuote => {
                        match ch {
                            '0' => {
                                self.next_string.push('\0');
                                self.quote_string.push('0');
                                self.state.set_char_point_or_quote();
                            }
                            't' => {
                                self.next_string.push('\t');
                                self.quote_string.push('t');
                                self.state.set_char_point_or_quote();
                            }
                            'n' => {
                                self.next_string.push('\n');
                                self.quote_string.push('n');
                                self.state.set_char_point_or_quote();
                            }
                            'r' => {
                                self.next_string.push('\r');
                                self.quote_string.push('r');
                                self.state.set_char_point_or_quote();
                            }
                            '\\' => {
                                self.next_string.push('\\');
                                self.quote_string.push('\\');
                                self.state.set_char_point_or_quote();
                            }
                            ch if is_symbol_continue(ch) => {
                                self.quote_string.push(ch);
                                self.state.set_quote();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::CharEnd => {
                        match ch {
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::Char(
                                        self.next_string
                                        .chars()
                                        .next()
                                        .unwrap()
                                    ));
                                self.next_string.clear();
                                self.quote_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::FirstDigits => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                            }
                            '.' => {
                                self.next_string.push(ch);
                                self.state.set_dot();
                            }
                            'E' | 'e' => {
                                self.next_string.push(ch);
                                self.state.set_exp();
                            }
                            '+' | '-' => todo!("Complex numbers not yet implemented"),
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::Number(
                                        NumType::Int(
                                            i32::from_str(&self.next_string)
                                                .unwrap()
                                        )));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    // float
                    ScannerState::Dot => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                                self.state.set_2nd_digits();
                            }
                            'E' | 'e' => {
                                self.next_string.push(ch);
                                self.state.set_exp();
                            }
                            '-' | '+' => todo!("Complex numbers not yet implemented"),
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::Number(
                                        NumType::Float(
                                            f32::from_str(&self.next_string)
                                            .unwrap()
                                        )));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::SecondDigits => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                            }
                            'E' | 'e' => {
                                self.next_string.push(ch);
                                self.state.set_exp();
                            }
                            '+' | '-' => todo!("Complex numbers not yet implemented"),
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::Number(
                                        NumType::Float(
                                            f32::from_str(&self.next_string)
                                            .unwrap()
                                        )));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::Exp => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                                self.state.set_exp_digits();
                            }
                            '+' | '-' => {
                                self.next_string.push(ch);
                                self.state.set_exp_sign();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::ExpSign => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                                self.state.set_exp_digits();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::ExpDigits => {
                        match ch {
                            '0'..='9' => {
                                self.next_string.push(ch);
                                self.state.set_exp_digits();
                            }
                            '+' | '-' => todo!("Complex numbers not yet implemented"),
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::Number(
                                        NumType::Float(
                                            f32::from_str(&self.next_string)
                                            .unwrap()
                                        )));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    // symbol scanning
                    ScannerState::Symbol => {
                        match ch {
                            ch if is_symbol_continue(ch) => {
                                self.next_string.push(ch);
                            }
                            ch if self.start(ch) => {
                                self.token_stream.push(Token::Symbol(self.next_string.clone()));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::Quote => {
                        match ch {
                            ch if is_symbol_continue(ch) => {
                                self.quote_string.push(ch);
                            }
                            ch if self.start(ch) => {
                                self.token_stream.push(Token::Quote(self.quote_string.clone()));
                                self.quote_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    // string string
                    ScannerState::String => {
                        match ch {
                            '"' => {
                                self.state.set_string_end();
                            }
                            '\\' => {
                                self.state.set_string_escape();
                            }
                            _ => self.next_string.push(ch),
                        }
                    }
                    ScannerState::StringEscape => {
                        match ch {
                            '0' => {
                                self.next_string.push('\0');
                                self.state.set_string();
                            }
                            't' => {
                                self.next_string.push('\t');
                                self.state.set_string();
                            }
                            'n' => {
                                self.next_string.push('\n');
                                self.state.set_string();
                            }
                            'r' => {
                                self.next_string.push('\r');
                                self.state.set_string();
                            }
                            '\\' => {
                                self.next_string.push('\\');
                                self.state.set_string();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::StringEnd => {
                        match ch {
                            ch if self.start(ch) => {
                                self.token_stream.push(
                                    Token::String(
                                        self.next_string.clone()
                                    ));
                                self.next_string.clear();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                }
            }
            match self.paren_count.cmp(&0) {
                std::cmp::Ordering::Less => panic!("Unbalanced Parenthesis"),
                std::cmp::Ordering::Equal => return,
                std::cmp::Ordering::Greater => {
                    print!("...\t");
                    std::io::stdout().flush().unwrap();
                }
            }
            buf.clear();
        }
    }

    fn start(&mut self, ch: char) -> bool {
        match ch {
            // Delimits
            '(' => {
                self.paren_count += 1;
                self.state.set_l_paren()
            }
            ')' => {
                if self.paren_count == 0 {
                    self.state.set_error(0);
                }
                self.paren_count -= 1;
                self.state.set_r_paren()
            }
            ';' => self.state.set_comment(),
            ch if ch.is_whitespace() => self.state.set_start(),
            _ => return false,
        }
        true
    }

    fn full_start(&mut self, ch: char) {
        match ch {
            // Char
            '\'' => {
                self.state.set_char_or_quote();
            }
            
            // Number or Symbol
            '+' | '-' => {
                self.next_string.push(ch);
                self.state.set_sign_or_symbol();
            }
            '0'..='9' => {
                self.next_string.push(ch);
                self.state.set_first_digits();
            }
            'N' | 'n' => {
                self.next_string.push(ch);
                self.state.set_nan_or_symbol(0);
            }
            'I' | 'i' => {
                self.next_string.push(ch);
                self.state.set_inf_or_symbol(0);
            }
            '.' => {
                self.next_string.push(ch);
                self.state.set_dot_or_digits();
            }
            // Symbol
            ch if is_symbol_start(ch) => {
                self.next_string.push(ch);
                self.state.set_symbol();
            }
            // String
            '"' => {
                self.state.set_string();
            }
            // skip
            ch if self.start(ch) => (),
            _ => (),
        }
    }

    fn debug(&self) {
        println!("Scanner Debug: {:?}", self)
    }
}

#[derive(Debug)]
enum ScannerState {
    Comment,
    Error(i32),
    Start,

    LParen,
    RParen,

    SignOrSymbol,
    NaNOrSymbol(u8),
    InfOrSymbol(u8),
    DotOrDigits,

    CharOrQuote,
    CharPoint,
    CharPointOrQuote,
    CharEscapeOrQuote,
    CharEnd,

    FirstDigits,

    Dot,
    SecondDigits,
    Exp,
    ExpSign,
    ExpDigits,

    Symbol,
    Quote,

    String,
    StringEscape,
    StringEnd,
}

impl ScannerState {
    fn new() -> Self {
        Self::Start
    }
    // main control
    fn set_start(&mut self) {
        *self = Self::Start
    }
    fn set_l_paren(&mut self) {
        *self = Self::LParen
    }
    fn set_r_paren(&mut self) {
        *self = Self::RParen
    }
    fn set_comment(&mut self) {
        *self = Self::Comment
    }
    fn set_error(&mut self, err_code: i32) {
        *self = Self::Error(err_code)
    }
    // intermediate
    fn set_sign_or_symbol(&mut self) {
        *self = Self::SignOrSymbol
    }
    fn set_nan_or_symbol(&mut self, step: u8) {
        *self = Self::NaNOrSymbol(step)
    }
    fn set_inf_or_symbol(&mut self, step: u8) {
        *self = Self::InfOrSymbol(step)
    }
    fn set_dot_or_digits(&mut self) {
        *self = Self::DotOrDigits
    }
    // char
    fn set_char_or_quote(&mut self) {
        *self = Self::CharOrQuote
    }
    fn set_char_point(&mut self) {
        *self = Self::CharPoint
    }
    fn set_char_point_or_quote(&mut self) {
        *self = Self::CharPointOrQuote
    }
    fn set_char_escape_or_quote(&mut self) {
        *self = Self::CharEscapeOrQuote
    }
    fn set_char_end(&mut self) {
        *self = Self::CharEnd
    }
    // number
    fn set_first_digits(&mut self) {
        *self = Self::FirstDigits
    }
    fn set_dot(&mut self) {
        *self = Self::Dot
    }
    fn set_2nd_digits(&mut self) {
        *self = Self::SecondDigits
    }
    fn set_exp(&mut self) {
        *self = Self::Exp
    }
    fn set_exp_sign(&mut self) {
        *self = Self::ExpSign
    }
    fn set_exp_digits(&mut self) {
        *self = Self::ExpDigits
    }
    // symbol
    fn set_symbol(&mut self) {
        *self = Self::Symbol
    }
    fn set_quote(&mut self) {
        *self = Self::Quote
    }
    // string
    fn set_string(&mut self) {
        *self = Self::String
    }
    fn set_string_escape(&mut self) {
        *self = Self::StringEscape
    }
    fn set_string_end(&mut self) {
        *self = Self::StringEnd
    }
}

fn is_symbol_start(ch: char) -> bool {
    match ch {
        'A'..='Z' |
        'a'..='z' |
        '_' |
        '-' |
        '+' |
        '*' |
        '/' => true,
        _ => false,
    }
}

fn is_symbol_continue(ch: char) -> bool {
    match ch {
        'A'..='Z' |
        'a'..='z' |
        '_' |
        '-' |
        '+' |
        '*' |
        '/' => true,
        _ => false,
    }
}

fn is_nan(ch: char, step: u8) -> bool {
    match step {
        0 => match ch {
            'A' | 'a' => true,
            _ => false,
        }
        1 => match ch {
            'N' | 'n' => true,
            _ => false,
        }
        _ => false,
    }
}

fn is_inf(ch: char, step: u8) -> bool {
    match step {
        0 => match ch {
            'N' | 'n' => true,
            _ => false,
        }
        1 => match ch {
            'F' | 'f' => true,
            _ => false,
        }
        2 => match ch {
            'I' | 'i' => true,
            _ => false,
        }
        3 => match ch {
            'N' | 'n' => true,
            _ => false,
        }
        4 => match ch {
            'I' | 'i' => true,
            _ => false,
        }
        5 => match ch {
            'T' | 't' => true,
            _ => false,
        }
        6 => match ch {
            'Y' | 'y' => true,
            _ => false,
        }
        _ => false,
    }
}
