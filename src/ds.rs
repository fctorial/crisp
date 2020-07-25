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
    Symbol(usize),
    Bool(bool),
    Int(i64),
    Float(f64),
    List(LList),
    ////
    Lambda(ft),
    Macro(ft),
    Undefined,
    RecurFlag(LList),
}
impl Default for Value {
    fn default() -> Self {
        Value::Undefined
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Symbol(idx) => f.write_str(format!("{}", idx).as_str()),
            Value::Bool(b) => f.write_str(if *b { "true" } else { "false" }),
            Value::List(v) => (v as &dyn Display).fmt(f),
            Value::Int(v) => (v as &dyn Display).fmt(f),
            Value::Float(v) => (v as &dyn Display).fmt(f),
            Value::Undefined => f.write_str("null"),
            Value::Lambda(_F) => f.write_str("<function>"),
            Value::Macro(_F) => f.write_str("<function>"),
            Value::RecurFlag(_l) => f.write_str("<recur>")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct _LList<T: Clone>(Option<Arc<(T, _LList<T>)>>);

pub type LList = _LList<Value>;

impl<T: Clone> _LList<T> {
    pub fn empty() -> Self {
        _LList(None)
    }

    pub fn cons(&self, v: T) -> Self {
        _LList(Some(Arc::new((v, match &self.0 {
            None => _LList::empty(),
            Some(ptr) => _LList(Some(ptr.clone())),
        }))))
    }

    pub fn iter(&self) -> LIterator<T> {
        LIterator(self.clone())
    }

    pub fn first(&self) -> Option<T> {
        match &self.0 {
            None => None,
            Some(ptr) => Some(ptr.0.clone()),
        }
    }

    pub fn rest(&self) -> Option<Self> {
        match &self.0 {
            None => None,
            Some(ptr) => Some(ptr.1.clone()),
        }
    }

    pub fn rest_t(&self) -> Self {
        match &self.rest() {
            None => _LList::empty(),
            Some(ptr) => ptr.to_owned(),
        }
    }

    pub fn nth(&self, i: usize) -> Option<T> {
        if i == 0 {
            self.0.as_ref().map(|ptr| ptr.0.clone())
        } else {
            match self.rest() {
                None => None,
                Some(l) => l.nth(i - 1)
            }
        }
    }
}

impl<T: Clone> Clone for _LList<T> {
    fn clone(&self) -> Self {
        match &self.0 {
            None => _LList(None),
            Some(ptr) => _LList(Some(ptr.clone())),
        }
    }
}

impl<T: Clone> FromIterator<T> for _LList<T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let mut values = vec![];
        for v in iter {
            values.push(v);
        }
        values.iter().rev().fold(_LList::empty(), |l, e| l.cons(e.to_owned()))
    }
}

pub struct LIterator<T: Clone>(_LList<T>);

impl<T: Clone> Iterator for LIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match &self.0 {
            _LList(None) => None,
            _LList(Some(ptr)) => {
                let res = ptr.0.clone();
                self.0 = ptr.1.clone();
                Some(res)
            }
        }
    }
}

impl<T: Clone + Display> Display for _LList<T> {
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
        self.0.iter().find(|(kk, _v)| kk == k).map(|(_k, v)| v)
    }

    pub fn set(&mut self, k: K, v: V) {
        match self.0.iter().zip(0..1000).find(|((kk, _), _)| *kk == k) {
            None => self.0.push((k, v)),
            Some(((_, _), idx)) => { std::mem::replace(&mut self.0[idx], (k, v)); }
        };
    }
}

impl<K: Clone + PartialEq, V: Clone> Clone for ArrayMap<K, V> {
    fn clone(&self) -> Self {
        ArrayMap(self.0.clone())
    }
}
