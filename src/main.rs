#![feature(assoc_int_consts)]

mod ds;
mod vm;
mod parser;

use ds::*;
use ds::Value::*;
use std::string::{ToString, String};
use std::collections::hash_map::DefaultHasher;
use std::default::Default;
use std::prelude::v1::Vec;
use std::iter::{Iterator, IntoIterator};
use std::borrow::BorrowMut;
use rutils::*;
use crate::parser::{toks, Token, parse_list, parse};
use std::slice::{Iter, IterMut};
use std::option::Option;
use std::result::Result;
use std::result::Result::Err;
use std::convert::AsRef;
use crate::vm::{eval, Bindings};
use std::clone::Clone;
use std::collections::HashMap;

fn parse_code(s : &str) -> Value {
    let chars: Vec<char> = s.chars().collect();
    let mut toks = toks(chars.iter().peekable().borrow_mut());
    parse(&mut toks.into_iter().peekable()).unwrap()
}

fn eb() -> Bindings {
    HashMap::new()
}

fn main() {
    let mut vbs = eb();
    let mut lbs1 = eb();
    let mut lbs2 = eb();
    vbs.insert("a".to_string(), Int(1));
    vbs.insert("b".to_string(), Int(2));
    lbs1.insert("b".to_string(), Int(3));
    lbs2.insert("b".to_string(), Int(4));
    let code = parse_code("b");

    let res = eval(&code, &mut vbs, &mut vec![]);
    debug_(res);
}
