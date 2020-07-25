#![feature(assoc_int_consts)]
#![feature(core_intrinsics)]
// #![allow(warnings)]

mod ds;
mod vm;
mod parser;

use ds::*;

use std::prelude::v1::Vec;
use std::iter::{Iterator, IntoIterator};
use std::borrow::BorrowMut;
use rutils::*;
use crate::parser::{toks, parse, ParserError};
use crate::parser::ParserError::*;

use crate::vm::{eval, Bindings};

use std::collections::HashMap;
use std::result::Result::*;

use std::result::Result;
use std::time::SystemTime;


fn parse_exp(s: &str) -> Result<Value, ParserError> {
    let chars: Vec<char> = s.chars().collect();
    let toks = toks(chars.iter().peekable().borrow_mut());
    let mut titer = toks.into_iter();
    parse(&mut titer)
}

fn parse_all(s: &str) -> Result<Vec<Value>, ParserError> {
    let chars: Vec<char> = s.chars().collect();
    let toks = toks(chars.iter().peekable().borrow_mut());
    let mut titer = toks.into_iter();
    let mut vs = vec![];
    loop {
        let exp = parse(&mut titer);
        match exp {
            Ok(v) => vs.push(v),
            Err(e) => match e {
                NoTokens => break,
                _ => return Err(e)
            },
        }
    }
    Ok(vs)
}

fn eb() -> Bindings {
    HashMap::new()
}

fn run_code(code: &str) {
    let mut vbs = eb();
    let res = parse_all(code).unwrap().iter()
        .map(|e| {
            eval(&e, &mut vbs, &mut vec![])
        })
        .last();
    debug_(res);
}

fn t_bind() {
    run_code("\
    (bindl ((a 1)\
            (b a))\
        b)\
     ");
}

fn t_loop(i: usize) {
    run_code(format!("\
    (set n {})
    (loop ((i 1)
           (res 0))
      (if (= i n)
        res
        (recur (+ i 1) (+ res i))))
    ", i).as_str())
}

fn main() {
    f();
    // let tm = SystemTime::now();
    // let mut vbs = eb();
    // let mut lbs_s = vec![ArrayMap::new(10)];
    // for i in 0..=std::env::args().nth(1).unwrap().parse().unwrap() {
    //     Set(LList::empty().cons(Int(1000000)).cons(Symbol("a".to_string())), &mut vbs, &mut lbs_s);
    // }
    // match tm.elapsed() {
    //     Err(e) => {},
    //     Ok(s) => debug_(s.as_micros())
    // };
}

fn f() {
    let tm = SystemTime::now();
    t_loop(std::env::args().nth(1).unwrap().parse().unwrap());
    match tm.elapsed() {
        Err(_e) => {},
        Ok(s) => debug_(s.as_millis())
    };
}
