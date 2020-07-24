#![feature(assoc_int_consts)]
#![feature(core_intrinsics)]
#![allow(warnings)]

mod ds;
mod vm;
mod parser;

use ds::*;
use ds::Value::*;
use std::string::{ToString, String};


use std::prelude::v1::Vec;
use std::iter::{Iterator, IntoIterator};
use std::borrow::BorrowMut;
use rutils::*;
use crate::parser::{toks, parse, ParserError};
use crate::parser::ParserError::*;

use crate::vm::{eval, Bindings, Set};

use std::collections::HashMap;
use std::result::Result::*;
use std::intrinsics::size_of;
use std::result::Result;
use std::time::SystemTime;
use std::option::Option::{Some, None};

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

fn t_vars() {
    let mut vbs = eb();
    let mut lbs1 = eb();
    let mut lbs2 = eb();
    vbs.insert("a".to_string(), Int(1));
    vbs.insert("b".to_string(), Int(2));
    lbs1.insert("b".to_string(), Int(3));
    lbs2.insert("b".to_string(), Int(4));

    if let Ok(code) = parse_exp("b") {
        let res = eval(&code, &mut vbs, &mut vec![]);
        debug_(res);
    }
}

fn t_bool() {
    let mut vbs = eb();
    if let Ok(code) = parse_exp(CODE) {
        let _res = eval(&code, &mut vbs, &mut vec![]);
        debug_(vbs);
    }
}

static CODE: &str = "\
    (set b false)\
    (if b 1 2)\
    ";

fn t_all() {
    let mut vbs = eb();
    let lr = parse_all(CODE).unwrap().iter()
        .map(|e| {
            eval(&e, &mut vbs, &mut vec![])
        })
        .last();
    debug_(lr);
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
        Err(e) => {},
        Ok(s) => debug_(s.as_millis())
    };
}
