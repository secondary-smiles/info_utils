use crate::{error};

pub trait EvalResult<T, E> {
    fn eval(self) -> T where E: std::fmt::Debug;
    fn eval_or(self, sub: T) -> T;
    fn eval_or_default(self) -> T where T: Default;
    fn eval_or_else<F>(self, func: F) -> T where F: FnOnce() -> T;
}

impl<T, E: std::fmt::Debug> EvalResult<T, E> for Result<T, E> {
    fn eval(self) -> T where E: std::fmt::Debug,
    {
        match self {
            Ok(v) => v,
            Err(e) => error!(format!("{:?}", e))
        }
    }

    fn eval_or(self, sub: T) -> T {
        match self {
            Ok(v) => v,
            Err(_) => sub
        }
    }

    fn eval_or_default(self) -> T where T: Default {
        match self {
            Ok(v) => v,
            Err(_) => Default::default()
        }
    }

    fn eval_or_else<F>(self, func: F) -> T where F: FnOnce() -> T {
        match self {
            Ok(v) => v,
            Err(_) => func(),
        }
    }
}