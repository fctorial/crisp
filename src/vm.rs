use crate::ds::*;
use std::collections::HashMap;
use std::ops::Fn;
use std::option::Option::*;
use std::string::{String, ToString};
use std::result::Result;
use std::result::Result::*;
use rutils::fail;
use lazy_static::lazy_static;
use crate::ds::Value::{Undefined, Symbol, List, Macro, Lambda};
use std::iter::{Iterator, IntoIterator};
use std::clone::Clone;
use std::prelude::v1::Vec;

pub type Bindings = HashMap<String, Value>;

macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value as fn(LList) -> Result<Value, String>);
            )+
            m
        }
     };
);
lazy_static! {
     static ref sfs : HashMap<&'static str, ft> = map!{
        "p" => Print
        // "set" => Set
        // "bindl" => BindL,
        // "lambda" => Lambda,
        // "macro" => Macro
        // "do" => Do,
        // "if" => If,
        // "loop" => Loop,
        // "quote" => Quote,
        // "unquote" => Unquote,
        // "read" => Read
    };
}


// fn LLambda(mut l: LList, &mut bs: Bindings) -> Result<Value, String> {
//     let args = l.first();
//     let body = l.rest_t();
//     if let Some(args_value) = args {
//         if let List(args_list) = args_value {
//             Ok(Lambda(|l| {
//
//             }))
//         } else {
//             Err("Invalid Args list".to_string())
//         }
//     } else {
//         Err("Args list not provided to lambda".to_string())
//     }
// }
//
// fn Set(mut l: LList, &mut bs: Bindings) -> Result<Value, String> {
//     let s = l.first();
//     let value = l.rest_t().first();
//     if let Some(Symbol(w)) = l.first() {
//         if let Some(v) = value {
//             eval(v, bs)
//         } else {
//             Err("Invalid Args to set: Invalid value".to_string())
//         }
//     } else {
//         Err("Invalid Args to set: Invalid symbol".to_string())
//     }
// }

fn Print(l: LList) -> Result<Value, String> {
    for e in l.iter() {
        print!("{} ", e);
    }
    println!();
    Ok(Value::Undefined)
}
// fn Lambda(mut l: LList) -> Result<Value, String> {
//     if let Some(Symbol(w)) = l.first() {
//         if w == "lambda" {
//             l = l.rest()?;
//             if let Some(List(args)) = l.first() {
//                 let args_valid = l.iter().fold(true, |res, e| res && if let Symbol(s) = e { true } else { false });
//                 let body = l.rest()?;
//             }
//         }
//     }
//     Ok(Null)
// }

pub fn eval(v: &Value, bs: &mut Bindings, lbs_s: &mut Vec<Bindings>) -> Result<Value, String> {
    match v {
        Symbol(w) => {
            let local = lbs_s.iter().rev()
                .map(|lbs| lbs.get(w))
                .filter(|r| if let Some(v) = r { true } else { false })
                .next()
                .map(|o| o.unwrap());
            match local {
                None => match bs.get(w) {
                    None => Err(format!("unknown identifier: {:?}", w)),
                    Some(v) => Ok(v.clone())
                },
                Some(v) => Ok(v.clone())
            }
        },
        List(l) => Err("todo".to_string()),
        _ => Ok(v.clone())
    }
}

pub fn execute(exp: &LList, vbs: &mut Bindings, lbs_s: &mut Vec<Bindings>) -> Result<Value, String> {
    match exp.first() {
        None => Ok(List(LList::empty())),
        Some(f) => match &f {
            Value::Symbol(s) => match sfs.get(s.as_str()) {
                Some(f) => f(exp.rest_t()),
                None => {
                    eval(&f, vbs, lbs_s)
                        .map(|v: Value| match v {
                            Lambda(l) => {
                                let rs = exp.rest_t();
                                let mut args:Vec<Value> = vec![];
                                for e in rs.iter() {
                                    args.push(eval(&e, vbs, lbs_s)?)
                                }
                                l(args.into_iter().collect())
                            },
                            Macro(m) => match m(exp.rest_t()) {
                                Err(s) => Err(s),
                                Ok(v) => eval(&v, vbs, lbs_s)
                            },
                            _ => Err(format!("can't call: {:?}", v)),
                        })?
                }
            },
            Value::List(l) => match eval(&List(l.clone()), vbs, lbs_s) {
                Err(s) => Err(s),
                Ok(f) => match execute(&exp.rest_t().cons(f), vbs, lbs_s) {
                    Err(s) => Err(s),
                    Ok(res) => Ok(res)
                }
            },
            Value::Undefined => Err("calling undefined var".to_string()),
            _ => Err(format!("can't call: {}", f))
        },
    }
}