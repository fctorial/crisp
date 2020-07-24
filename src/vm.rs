use crate::ds::*;
use std::collections::HashMap;

use std::option::Option::*;
use std::string::{String, ToString};
use std::result::Result;
use std::result::Result::*;

use lazy_static::lazy_static;
use crate::ds::Value::{Symbol, List, Macro, Lambda, Bool};
use std::iter::{Iterator, IntoIterator};
use std::clone::Clone;
use std::prelude::v1::Vec;

pub type Bindings = HashMap<String, Value>;

macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value as fn(LList, &mut Bindings, &mut Vec<Bindings>) -> Result<Value, String>);
            )+
            m
        }
     };
);
lazy_static! {
     static ref sfs : HashMap<&'static str, fn(LList, &mut Bindings, &mut Vec<Bindings>) -> Result<Value, String>> = map!{
        "set" => Set,
        // "bindl" => BindL,
        // "lambda" => Lambda,
        // "macro" => Macro
        // "do" => Do,
        "if" => If,
        // "loop" => Loop,
        // "quote" => Quote,
        // "read" => Read,
        "p" => Print
    };
}
fn err<T>(s: &str) -> Result<T, String> {
    Err(s.to_string())
}
// fn _t(args: LList, vbs : &mut Bindings, lbs_s : &mut Vec<Bindings>) -> Result<Value, String> {}
fn If(args: LList, vbs : &mut Bindings, lbs_s : &mut Vec<Bindings>) -> Result<Value, String> {
    let v = args.iter()
        .take(3)
        .collect::<Vec<Value>>();
    if v.len() < 3 {
        err("\"if\" needs at three arguments")
    } else {
        if let Bool(b) = eval(v.get(0).unwrap(), vbs, lbs_s)? {
            eval(v.get(if b {1} else {2}).unwrap(), vbs, lbs_s)
        } else {
            err("first arg to \"if\" must evaluate to a bool")
        }
    }
}

fn Set(args: LList, vbs : &mut Bindings, lbs_s : &mut Vec<Bindings>) -> Result<Value, String> {
    let v = args.iter()
        .take(2)
        .collect::<Vec<Value>>();
    if v.len() != 2 {
        err("\"set\" needs two arguments")
    } else {
        if let Symbol(w) = v.get(0).unwrap() {
            let value = eval(v.get(1).unwrap(), vbs, lbs_s)?;
            vbs.insert(w.to_string(), value.clone());
            Ok(value)
        } else {
            err("first arg to \"set\" must be a symbol")
        }
    }
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

pub fn eval(v: &Value, vbs: &mut Bindings, lbs_s: &mut Vec<Bindings>) -> Result<Value, String> {
    match v {
        Symbol(w) => {
            let local = lbs_s.iter().rev()
                .map(|lbs| lbs.get(w))
                .filter(|r| if let Some(_v) = r { true } else { false })
                .next()
                .map(|o| o.unwrap());
            match local {
                None => match vbs.get(w) {
                    None => Err(format!("unknown identifier: {:?}", w)),
                    Some(v) => Ok(v.clone())
                },
                Some(v) => Ok(v.clone())
            }
        },
        List(l) => execute(l, vbs, lbs_s),
        _ => Ok(v.clone())
    }
}

pub fn execute(exp: &LList, vbs: &mut Bindings, lbs_s: &mut Vec<Bindings>) -> Result<Value, String> {
    match exp.first() {
        None => Ok(List(LList::empty())),
        Some(f) => match &f {
            Value::Symbol(s) => match sfs.get(s.as_str()) {
                Some(f) => f(exp.rest_t(), vbs, lbs_s),
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

fn Print(_args: LList, _vbs : &mut Bindings, _lbs_s : &mut Vec<Bindings>) -> Result<Value, String> {err("")}
