#![feature(assoc_int_consts)]

mod ds;
// mod vm;
mod parser;

use ds::*;
use ds::Value::*;
use std::string::{ToString, String};
use std::collections::hash_map::DefaultHasher;
use std::default::Default;
use std::prelude::v1::Vec;
use std::iter::{Iterator, IntoIterator};
use std::borrow::BorrowMut;
use rutils::debug;
use crate::parser::{toks, Token, parse_list, parse};
use std::slice::{Iter, IterMut};
use std::option::Option;
use std::result::Result;
use std::result::Result::Err;
use std::convert::AsRef;

fn main() {
    let s = "(inc(inc  23))";
    let chars: Vec<char> = s.chars().collect();
    let mut toks = toks(chars.iter().peekable().borrow_mut());
    debug(&toks);
    let v = parse(&mut toks.into_iter().peekable());
    debug(&v);
}
