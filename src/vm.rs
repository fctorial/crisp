use crate::ds::*;
use std::collections::HashMap;
use std::ops::Fn;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

static sfs : HashMap<&str, fn(LList)->Value> = map!{
    "set" => Set,
    "setl" => SetL,
    "lambda" => Lambda,
    "do" => Do,
    "if" => If,
    "loop" => Loop,
    "quote" => Quote,
    "unquote" => Unquote,
    "read" => Read
};

pub fn eval(exp : LList) -> Value {
    Value::Null
}