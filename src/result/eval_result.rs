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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let foo: Result<u16, &str> = Ok(7);
        assert_eq!(foo.eval(), 7);
    }

    #[test]
    fn test_eval_or() {
        let mut foo: Result<u16, &str> = Ok(7);
        assert_eq!(foo.eval_or(3), 7);

        foo = Err("error");
        assert_eq!(foo.eval_or(3), 3);
    }

    #[test]
    fn test_eval_or_default() {
        let mut foo: Result<u16, &str> = Ok(7);
        assert_eq!(foo.eval_or_default(), 7);

        foo = Err("error");
        assert_eq!(foo.eval_or_default(), 0);
    }

    #[test]
    fn test_eval_or_else() {
        let mut foo: Result<u16, &str> = Ok(7);
        assert_eq!(foo.unwrap_or_else(|s| s.len() as u16), 7);

        foo = Err("error");
        assert_eq!(foo.unwrap_or_else(|s| s.len() as u16), 5);
    }
}