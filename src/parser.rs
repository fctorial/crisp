use std::string::{String, ToString};
use crate::ds::{Value, LList};
use crate::ds::Value::{Null, List, Symbol};
use std::iter::{IntoIterator, Iterator, Peekable};
use std::result::Result;
use std::result::Result::{Err, Ok};
use std::option::Option::{Some, None};
use std::prelude::v1::Vec;
use std::option::Option;
use std::slice::Iter;
use rutils::debug;
use std::borrow::ToOwned;
use std::vec::IntoIter;

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

pub fn parse(toks : &mut Peekable<IntoIter<Token>>) -> Result<Value, String> {
    use Token::*;
    match toks.next() {
        Some(t) => match t {
            PAREN1 => parse_list(toks),
            WORD(w) => parse_word(w),
            PAREN2 => Err("Unexpected ')'".to_string())
        },
        None => Err("no tokens".to_string())
    }
}

pub fn parse_list(toks: &mut Peekable<IntoIter<Token>>) -> Result<Value, String> {
    use Token::*;
    let mut values = vec![];
    loop {
        match toks.next() {
            None => return Err("unexpected EOF".to_string()),
            Some(t) => match t {
                PAREN1 => {
                    let inner = parse_list(toks);
                    match inner {
                        Ok(v) => values.push(v),
                        Err(s) => return Err(s)
                    }
                },
                PAREN2 => break,
                WORD(w) => values.push(Symbol(w))
            }
        }
    }
    Ok(List(values.iter().rev().fold(LList::empty(), |l, e| l.cons(e.to_owned()))))
}

fn parse_word(w : String) -> Result<Value, String> {
    match w.chars().next().unwrap() {
        '0'..='9' => {
            w.parse()
                .map(|i| Value::Int(i))
                .map_err(|e| e.to_string())
        },
        _ => Ok(Value::Symbol(w))
    }
}
