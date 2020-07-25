#![feature(assoc_int_consts)]
#![feature(core_intrinsics)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![allow(warnings)]

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
use std::string::{ToString, String};
use std::ops::Fn;
use std::boxed::Box;


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
    let tm = SystemTime::now();
    let mut vbs = eb();
    let res = parse_all(code).unwrap().iter()
        .map(|e| {
            eval(&e, &mut vbs, &mut vec![])
        })
        .last();
    debug_(res);
    match tm.elapsed() {
        Err(_e) => {},
        Ok(s) => debug_(s.as_millis())
    };
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

fn t_lambda(n: usize) {
    run_code(format!("\
    (set fib (lambda (n)
                (if (< n 2)
                  n
                  (+ (fib (- n 1))
                     (fib (- n 2))))))
    (fib {})", n).as_str());
}

fn main() {
    t_lambda(std::env::args().nth(1).unwrap().parse().unwrap());
}

fn f() {
    let tm = SystemTime::now();
    t_loop(std::env::args().nth(1).unwrap().parse().unwrap());
    match tm.elapsed() {
        Err(_e) => {},
        Ok(s) => debug_(s.as_millis())
    };
}

