//!# Eval Option
//!
//! `eval()` implemented for the `Result<T, E>` type

use crate::{error};

/// Eval Option trait that enables `eval()` for the `Result<T, E>` type
pub trait EvalResult<T, E> {
    fn eval(self) -> T where E: std::fmt::Debug;
    fn eval_or(self, sub: T) -> T;
    fn eval_or_default(self) -> T where T: Default;
    fn eval_or_else<F>(self, func: F) -> T where F: FnOnce(E) -> T;
}

impl<T, E: std::fmt::Debug> EvalResult<T, E> for Result<T, E> {
    /// Drop-in for the `unwrap()` function
    ///
    /// returns `v` if `Ok()`; else calls an `error!`
    ///
    /// Example
    /// ```rust
    /// # use crate::log_utils::prelude::*;
    /// # fn main() {
    ///     let foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.eval(), 7);
    /// # }
    /// ```
    fn eval(self) -> T where E: std::fmt::Debug,
    {
        match self {
            Ok(v) => v,
            Err(e) => error!("{:?}", e)
        }
    }


    /// Drop-in for the `unwrap_or()` function
    ///
    /// returns `v` if `Ok()`; else returns `sub`
    ///
    /// Example
    /// ```rust
    /// # use crate::log_utils::prelude::*;
    /// # fn main() {     ///
    ///     let mut foo: Result<u16, &str> = Ok(7);    ///
    ///     assert_eq!(foo.eval_or(3), 7);
    ///
    ///     foo = Err("error");
    ///     assert_eq!(foo.eval_or(3), 3);
    /// # }
    /// ```
    fn eval_or(self, sub: T) -> T {
        match self {
            Ok(v) => v,
            Err(_) => sub
        }
    }

    /// Drop-in for the `unwrap_or_default()` function
    ///
    /// returns `v` if `Ok()`; else returns default value for type `T`
    ///
    /// Example
    /// ```rust
    /// # use crate::log_utils::prelude::*;
    /// # fn main() {
    ///     let mut foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.eval_or_default(), 7);
    ///     foo = Err("error");
    ///     assert_eq!(foo.eval_or_default(), 0);
    /// # }
    /// ```
    fn eval_or_default(self) -> T where T: Default {
        match self {
            Ok(v) => v,
            Err(_) => Default::default()
        }
    }

    /// Drop-in for the `unwrap_or_else()` function
    ///
    /// returns `v` if `Ok()`; else returns return value of function `func`
    ///
    /// Example
    /// ```rust
    /// # use crate::log_utils::prelude::*;
    /// # fn main() {
    ///     let mut foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.eval_or_else(|d| d.len() as u16), 7);
    ///
    ///     foo = Err("error");
    ///     assert_eq!(foo.eval_or_else(|d| d.len() as u16), 5);
    /// # }
    /// ```
    fn eval_or_else<F>(self, func: F) -> T where F: FnOnce(E) -> T {
        match self {
            Ok(v) => v,
            Err(e) => func(e),
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
        assert_eq!(foo.eval_or_else(|d| d.len() as u16), 7);

        foo = Err("error");
        assert_eq!(foo.eval_or_else(|d| d.len() as u16), 5);
    }
}