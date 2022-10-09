use crate::error;

pub trait EvalOption<T> {
    fn eval(self) -> T;
    fn eval_or(self, sub: T) -> T;
}

impl<T> EvalOption<T> for Option<T> {
    fn eval(self) -> T {
        match self {
            Some(v) => v,
            None => error!("called `Option::unwrap()` on a `None` value")
        }
    }

    fn eval_or(self, sub: T) -> T {
        match self {
            Some(v) => v,
            None => sub
        }
    }
}