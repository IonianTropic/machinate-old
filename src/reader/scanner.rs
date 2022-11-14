use std::io::{BufRead, Write};
use std::process::exit;
use std::str::FromStr;

use super::token::{Token, NumType};

const VERBOSE_DEBUG: bool = false;

/// Handwritten scanner without regular expressions
#[derive(Debug)]
pub struct Scanner {
    state: ScannerState,
    pub token_stream: Vec<Token>,
    next_string: String,
    paren_count: i32,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            state: ScannerState::new(),
            token_stream: Vec::new(),
            next_string: String::new(),
            paren_count: 0,
        }
    }

    pub fn scan<B: BufRead>(&mut self, input: &mut B) {
        let mut buf = String::new();
        'line: loop {
            match input.read_line(&mut buf) {
                Ok(_) => (),
                Err(len) => panic!("line length limit (80) reached: {}", len),
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
                    ScannerState::CharStart => {
                        match ch {
                            '\'' => self.state.set_error(0),
                            '\\' => self.state.set_char_escape(),
                            '\x20'..= '\u{d7ff}' |
                            '\u{e000}'..= '\u{10ffff}' => {
                                self.token_stream.push(Token::Char(ch));
                                self.state.set_char_point();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::CharPoint => {
                        match ch {
                            '\'' => {
                                self.state.set_start();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    ScannerState::CharEscape => {
                        // TODO implement escape sequences u, x
                        match ch {
                            '0' => {
                                self.token_stream.push(Token::Char('\0'));
                                self.state.set_char_point();
                            }
                            't' => {
                                self.token_stream.push(Token::Char('\t'));
                                self.state.set_char_point();
                            }
                            'n' => {
                                self.token_stream.push(Token::Char('\n'));
                                self.state.set_char_point();
                            }
                            'r' => {
                                self.token_stream.push(Token::Char('\r'));
                                self.state.set_char_point();
                            }
                            '\\' => {
                                self.token_stream.push(Token::Char('\\'));
                                self.state.set_char_point();
                            }
                            '\'' => {
                                self.token_stream.push(Token::Char('\''));
                                self.state.set_char_point();
                            }
                            _ => self.state.set_error(0),
                        }
                    }
                    // number scanning
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
                self.state.set_char_start();
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

    CharStart,
    CharPoint,
    CharEscape,

    FirstDigits,

    Dot,
    SecondDigits,
    Exp,
    ExpSign,
    ExpDigits,

    Symbol,
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
    fn set_char_start(&mut self) {
        *self = Self::CharStart
    }
    fn set_char_point(&mut self) {
        *self = Self::CharPoint
    }
    fn set_char_escape(&mut self) {
        *self = Self::CharEscape
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
