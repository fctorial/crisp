use std::option::Option;

use std::option::Option::*;
use std::string::{String};
use std::fmt::{Debug, Formatter, Write, Result, Display};
use std::result::Result::Ok;
use std::iter::{Iterator, FromIterator, IntoIterator};
use std::sync::Arc;
use std::default::Default;

use std::borrow::ToOwned;


use std::clone::Clone;
use std::prelude::v1::{Vec, PartialEq};

pub type ft = fn(LList) -> std::result::Result<Value, String>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Symbol(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    List(LList),
    ////
    Lambda(ft),
    Macro(ft),
    Undefined,
    RecurFlag(LList)
}
impl Default for Value {
    fn default() -> Self {
        Value::Undefined
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Symbol(name) => f.write_str(format!("{}", name).as_str()),
            Value::Bool(b) => f.write_str(if *b {"true"} else {"false"}),
            Value::List(v) => (v as &dyn Display).fmt(f),
            Value::Int(v) => (v as &dyn Display).fmt(f),
            Value::Float(v) => (v as &dyn Display).fmt(f),
            Value::Undefined => f.write_str("null"),
            Value::Lambda(_F) => f.write_str("<function>"),
            Value::Macro(_F) => f.write_str("<function>"),
            Value::RecurFlag(l) => f.write_str("<recur>")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LList(Option<Arc<(Value, LList)>>);
pub(crate) static EMPTY : LList = LList(None);
impl LList {
    pub fn empty() -> LList {
        LList(None)
    }

    pub fn _empty() -> &'static LList {
        &EMPTY
    }

    pub fn cons(&self, v: Value) -> LList {
        LList(Some(Arc::new((v, match &self.0 {
            None => LList::empty(),
            Some(ptr) => LList(Some(ptr.clone())),
        }))))
    }

    pub fn iter(&self) -> LIterator {
        LIterator(self.clone())
    }

    pub fn first(&self) -> Option<Value> {
        match &self.0 {
            None => None,
            Some(ptr) => Some(ptr.0.clone()),
        }
    }

    pub fn rest(&self) -> Option<LList> {
        match &self.0 {
            None => None,
            Some(ptr) => Some(ptr.1.clone()),
        }
    }

    pub fn rest_t(&self) -> LList {
        match &self.rest() {
            None => LList::empty(),
            Some(ptr) => ptr.to_owned(),
        }
    }

    pub fn nth(&self, i: usize) -> Option<Value> {
        if i == 0 {
            self.0.as_ref().map(|ptr| ptr.0.clone())
        } else {
            match self.rest() {
                None => None,
                Some(l) => l.nth(i-1)
            }
        }
    }
}

impl Clone for LList {
    fn clone(&self) -> Self {
        match &self.0 {
            None => LList(None),
            Some(ptr) => LList(Some(ptr.clone())),
        }
    }
}

impl FromIterator<Value> for LList {
    fn from_iter<T: IntoIterator<Item=Value>>(iter: T) -> Self {
        let mut values = vec![];
        for v in iter {
            values.push(v);
        }
        values.iter().rev().fold(LList::empty(), |l, e| l.cons(e.to_owned()))
    }
}

pub struct LIterator(LList);
impl Iterator for LIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        match &self.0 {
            LList(None) => None,
            LList(Some(ptr)) => {
                let res = ptr.0.clone();
                self.0 = ptr.1.clone();
                Some(res)
            }
        }
    }
}

impl Display for LList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = String::new();
        s.push('(');
        s.push(' ');
        for v in self.iter() {
            s.push_str(format!("{}", v).as_str());
            s.push(' ');
        }
        s.push(')');
        f.write_str(&*s);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ArrayMap<K: PartialEq, V>(Vec<(K, V)>);

impl<K: PartialEq, V> ArrayMap<K, V> {
    pub fn new(n: usize) -> Self {
        ArrayMap(Vec::with_capacity(n))
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.0.iter().find(|(kk, v)| kk == k).map(|(k, v)| v)
    }

    pub fn set(&mut self, k: K, v: V) {
        match self.0.iter().zip(0..1000).find(|((kk, _), _)| *kk == k) {
            None => self.0.push((k, v)),
            Some(((_, _), idx)) => { std::mem::replace(&mut self.0[idx], (k, v)); }
        };
    }
}
