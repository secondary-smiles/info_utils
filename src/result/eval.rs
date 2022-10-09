use crate::{err};

pub trait Eval<T, E> {
    fn eval(self) -> T;
    fn eval_or(self, sub: T) -> T where E: std::fmt::Debug;
}

impl<T, E: std::fmt::Debug> Eval<T, E> for Result<T, E> {
    fn eval(self) -> T
    where E: std::fmt::Debug,
    {
        match self {
            Ok(v) => v,
            Err(e) => err!(format!("{:#?}", e))
        }
    }

    fn eval_or(self, sub: T) -> T {
        match self {
            Ok(v) => v,
            Err(_) => sub
        }
    }
}