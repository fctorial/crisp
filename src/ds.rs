use std::option::Option;
use std::rc::Rc;
use std::option::Option::*;
use std::string::String;
use std::clone::Clone;
use std::fmt::{Debug, Formatter, Write, Result, Display};
use std::result::Result::Ok;
use std::iter::Iterator;
use std::sync::Arc;
use std::default::Default;
use crate::ds::Value::Null;

#[derive(Clone)]
pub enum Value {
    Symbol(String),
    Int(i128),
    Float(f64),
    List(LList),
    Null
}
impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Symbol(name) => f.write_str(format!("{}", name).as_str()),
            Value::List(v) => (v as &Display).fmt(f),
            Value::Int(v) => (v as &Display).fmt(f),
            Value::Float(v) => (v as &Display).fmt(f),
            Value::Null => f.write_str("null")
        }
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (self as &Display).fmt(f)
    }
}

pub struct LList(Option<Arc<(Value, LList)>>);
pub(crate) static EMPTY : LList = LList(None);
impl LList {
    pub fn empty() -> LList {
        LList(None)
    }

    pub fn cons(&self, v: Value) -> LList {
        LList(Some(Arc::new((v, match &self.0 {
            None => LList::empty(),
            Some(ptr) => LList(Some(ptr.clone())),
        }))))
    }

    fn iter(&self) -> LIterator {
        LIterator(self.clone())
    }

    fn first(&self) -> Option<Value> {
        match &self.0 {
            None => None,
            Some(ptr) => Some(ptr.0.clone()),
        }
    }

    fn rest(&self) -> Option<LList> {
        match &self.0 {
            None => None,
            Some(ptr) => Some(ptr.1.clone()),
        }
    }

    fn nth(&self, i: usize) -> Option<Value> {
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

struct LIterator(LList);
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
impl Debug for LList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (self as &Display).fmt(f)
    }
}
