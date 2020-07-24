use std::string::{String, ToString};
use crate::ds::{Value, LList};
use crate::ds::Value::{Undefined, List, Symbol};
use std::iter::{Iterator, Peekable};
use std::result::Result;
use std::result::Result::{Err, Ok};
use std::option::Option::{Some, None};
use std::prelude::v1::Vec;

use std::slice::Iter;

use std::borrow::ToOwned;
use std::vec::IntoIter;
use crate::parser::ParserError::*;

#[derive(Debug)]
pub enum Token {
    PAREN1,
    PAREN2,
    WORD(String)
}

pub fn toks(chars : &mut Peekable<Iter<char>>) -> Vec<Token> {
    let mut res = vec![];
    loop {
        skip_whitespace(chars);
        match chars.peek() {
            Some(c) => match c {
                '(' => {
                    res.push(Token::PAREN1);
                    chars.next();
                },
                ')' => {
                    res.push(Token::PAREN2);
                    chars.next();
                },
                _ => res.push(Token::WORD(read_word(chars)))
            },
            None => break
        }
    }
    res
}

fn skip_whitespace(chars: &mut Peekable<Iter<char>>) {
    loop {
        match chars.peek() {
            None => return,
            Some(c) => {
                if ! c.is_whitespace() {
                    return;
                }
            }        }
        chars.next();
    }
}

fn is_w(c : char) -> bool {
    c.is_whitespace() || c == '(' || c == ')'
}

pub fn read_word(chars: &mut Peekable<Iter<char>>) -> String {
    let mut s = String::new();
    loop {
        match chars.peek() {
            None => return s,
            Some(c) => if is_w(**c) {
                return s;
            } else {
                s.push(**c);
            }
        }
        chars.next();
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedClosingParen,
    NoTokens,
    UnexpectedEof,
    InvalidInteger(String),
}

pub fn parse(toks: &mut IntoIter<Token>) -> Result<Value, ParserError> {
    use Token::*;
    match toks.next() {
        Some(t) => match t {
            PAREN1 => parse_list(toks),
            WORD(w) => parse_word(w),
            PAREN2 => Err(UnexpectedClosingParen)
        },
        None => Err(NoTokens)
    }
}

pub fn parse_list(toks: &mut IntoIter<Token>) -> Result<Value, ParserError> {
    use Token::*;
    let mut values = vec![];
    loop {
        match toks.next() {
            None => return Err(UnexpectedEof),
            Some(t) => match t {
                PAREN1 => {
                    let inner = parse_list(toks);
                    match inner {
                        Ok(v) => values.push(v),
                        Err(e) => return Err(e)
                    }
                },
                PAREN2 => break,
                WORD(w) => values.push(parse_word(w)?)
            }
        }
    }
    Ok(List(values.iter().rev().fold(LList::empty(), |l, e| l.cons(e.to_owned()))))
}

fn parse_word(w: String) -> Result<Value, ParserError> {
    use Value::*;
    match w.chars().next().unwrap() {
        '0'..='9' => {
            if w.contains('.') {
                match w.parse().map(|f| Float(f)) {
                    Ok(f) => return Ok(f),
                    _ => {}
                }
            }
            w.parse()
                .map(|i| Int(i))
                .map_err(|e| InvalidInteger(e.to_string()))
        },
        _ => {
            match w.parse().map(|b| Bool(b)) {
                Ok(b) => return Ok(b),
                _ => {}
            }
            return Ok(Symbol(w));
        }
    }
}
