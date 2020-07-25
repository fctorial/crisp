use crate::ds::*;
use std::collections::HashMap;

use std::option::Option::*;
use std::string::{String, ToString};
use std::result::Result;
use std::result::Result::*;

use lazy_static::lazy_static;
use crate::ds::Value::{Symbol, List, Macro, Lambda, Bool, Int, RecurFlag, Undefined};
use std::iter::{Iterator, IntoIterator};
use std::clone::Clone;
use std::prelude::v1::Vec;
use rutils::{debug_};

pub type Bindings = HashMap<String, Value>;
pub type LBindings = Vec<ArrayMap<String, Value>>;

macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value as fn(LList, &mut Bindings, &mut LBindings) -> Result<Value, String>);
            )+
            m
        }
     };
);
lazy_static! {
     pub static ref sfs : HashMap<&'static str, fn(LList, &mut Bindings, &mut LBindings) -> Result<Value, String>> = map!{
        "set" => Set,
        "bindl" => BindL,
        // "lambda" => Lambda,
        // "macro" => Macro
        "do" => Do,
        "if" => If,
        "loop" => Loop,
        "recur" => Recur,
        // "quote" => Quote,
        // "read" => Read,
        "p" => Print,
        "+" => Plus,
        "-" => Minus,
        "=" => Equal,
        "*" => Mult
    };
}
fn err<T>(s: &str) -> Result<T, String> {
    Err(s.to_string())
}

// fn _t(args: LList, vbs : &mut Bindings, lbs_s : &mut LBindings) -> Result<Value, String> {}
fn Do(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let mut last = Undefined;
    for exp in args.iter() {
        match eval(&exp, vbs, lbs_s) {
            Ok(r) => last = r,
            Err(e) => return Err(e),
        }
    }
    if last == Undefined {
        err("do error 1")
    } else {
        return Ok(last);
    }
}

fn Plus(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let mut res = 0;
    for e in args.iter() {
        match eval(&e, vbs, lbs_s) {
            Err(s) => return Err(s),
            Ok(e) => if let Int(i) = e {
                res += i;
            } else {
                debug_(e);
                return err("should be int");
            }
        }
    }
    Ok(Int(res))
}

fn Equal(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let val = match eval(&args.first().unwrap(), vbs, lbs_s) {
        Err(s) => return Err(s),
        Ok(v) => v
    };
    for e in args.rest_t().iter() {
        match eval(&e, vbs, lbs_s) {
            Err(s) => return Err(s),
            Ok(e) => if e != val {
                return Ok(Bool(false))
            }
        }
    }
    Ok(Bool(true))
}

fn Minus(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let mut res = 0;
    for e in args.iter() {
        match eval(&e, vbs, lbs_s) {
            Err(s) => return Err(s),
            Ok(e) => if let Int(i) = e {
                res -= i;
            } else {
                debug_(e);
                return err("should be int");
            }
        }
    }
    Ok(Int(res))
}

fn Mult(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let mut res = 1;
    for e in args.iter() {
        match eval(&e, vbs, lbs_s) {
            Err(s) => return Err(s),
            Ok(e) => if let Int(i) = e {
                res *= i;
            } else {
                debug_(e);
                return err("should be int");
            }
        }
    }
    Ok(Int(res))
}

fn Recur(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let mut ress = vec![];
    for res in args.iter().map(|exp| eval(&exp, vbs, lbs_s)) {
        match res {
            Err(s) => return Err(s),
            Ok(v) => ress.push(v)
        }
    }
    Ok(RecurFlag(ress.into_iter().collect()))
}

fn Loop(mut args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let bsv = args.first().unwrap();
    if let List(bs) = bsv {
        let bnames = bs.iter().map(|e| if let List(l) = e { l.first().unwrap() } else { panic!("") }).collect::<LList>();
        let n = bs.iter().count();
        loop {
            match eval(&List(args.cons(Symbol("bindl".to_string()))), vbs, lbs_s) {
                Ok(res) => match res {
                    RecurFlag(bs) => {
                        if bs.iter().count() != n {
                            return err("loop error 2");
                        }
                        args = args.rest_t().cons(List(bnames.iter().zip(bs.iter()).map(|(n, v)| List(vec![n, v].into_iter().collect())).collect::<LList>()))
                    }
                    _ => return Ok(res)
                },
                Err(e) => return Err(e)
            }
        }
    } else {
        return err("loop error 1");
    };
    loop {}
}

fn BindL(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    if let Some(List(bs)) = args.first() {
        let body = args.rest_t();
        if let None = body.rest() {
            return err("bindl error 5");
        }
        lbs_s.push(ArrayMap::new(5));
        for bv in bs.iter() {
            if let List(b) = bv {
                let vs = b.iter()
                    .take(2)
                    .collect::<Vec<Value>>();
                let sym = &vs[0];
                let value_exp = &vs[1];
                if let Symbol(_) = &sym {
                    match eval(&value_exp, vbs, lbs_s) {
                        Ok(value) => {
                            let mut lbs = lbs_s.pop().unwrap();
                            lbs.set(sym.to_string(), value);
                            lbs_s.push(lbs);
                        }
                        Err(s) => return Err(s),
                    }
                } else {
                    return err("bindl error 2");
                }
            } else {
                return err("bindl error 3");
            }
        }
        let mut last = Int(74);
        for exp in body.iter() {
            match eval(&exp, vbs, lbs_s) {
                Ok(r) => last = r,
                Err(e) => return Err(e),
            }
        }
        lbs_s.pop();
        return Ok(last);
    } else {
        err("bindl error1")
    }
}

fn If(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let v = args.iter()
        .take(3)
        .collect::<Vec<Value>>();
    if v.len() < 3 {
        err("\"if\" needs at three arguments")
    } else {
        if let Bool(b) = eval(v.get(0).unwrap(), vbs, lbs_s)? {
            eval(v.get(if b { 1 } else { 2 }).unwrap(), vbs, lbs_s)
        } else {
            err("first arg to \"if\" must evaluate to a bool")
        }
    }
}

fn CLambda(args: LList, _vbs: &mut Bindings, _lbs_s: &mut LBindings) -> Result<Value, String> {
    if let Some(_largs) = args.first() {} else {}
    Err("impl".to_string())
}

pub fn Set(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let v = args.iter()
        .take(2)
        .collect::<Vec<Value>>();
    if v.len() != 2 {
        err("\"set\" needs two arguments")
    } else {
        if let Symbol(w) = &v[0] {
            let value = eval(&v[1], vbs, lbs_s)?;
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

pub fn eval(v: &Value, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    match v {
        Symbol(w) => {
            let local = lbs_s.iter().rev()
                .map(|lbs| lbs.get(w))
                .filter(|r| if let Some(_v) = r { true } else { false })
                .next()
                .map(|o| o.unwrap());
            match local {
                None => match vbs.get(w) {
                    None => panic!(format!("unknown identifier: {:?}", w)),
                    Some(v) => Ok(v.clone())
                },
                Some(v) => Ok(v.clone())
            }
        }
        List(l) => execute(l, vbs, lbs_s),
        _ => Ok(v.clone())
    }
}

pub fn execute(exp: &LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
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
                                let mut args: Vec<Value> = vec![];
                                for e in rs.iter() {
                                    args.push(eval(&e, vbs, lbs_s)?)
                                }
                                l(args.into_iter().collect())
                            }
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

fn Print(args: LList, vbs: &mut Bindings, lbs_s: &mut LBindings) -> Result<Value, String> {
    let exp = args.first().unwrap();
    let res = eval(&exp, vbs, lbs_s)?;
    println!("{}", res);
    Ok(res)
}
