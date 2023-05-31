//!# Eval Option
//!
//! `eval()` implemented for the `Result<T, E>` type

use crate::errors;
use std::fmt::Display;

/// Eval Option trait that enables `eval()` for the `Result<T, E>` type
pub trait EvalResult<T, E> {
    fn eval(self) -> T
    where
        E: std::fmt::Debug;
    fn eval_or(self, sub: T) -> T;
    fn eval_or_default(self) -> T
    where
        T: Default;
    fn eval_or_else<F>(self, func: F) -> T
    where
        F: FnOnce(E) -> T;
    fn should<M>(self, panic: M) -> T
    where
        M: std::fmt::Display;
}

impl<T, E: std::fmt::Debug> EvalResult<T, E> for Result<T, E> {
    /// Drop-in for the `unwrap()` function
    ///
    /// returns `v` if `Ok()`; else calls an `errors!`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    ///     let foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.eval(), 7);
    /// # }
    /// ```
    fn eval(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            Ok(v) => v,
            Err(e) => errors!("{:?}", e),
        }
    }

    /// Drop-in for the `unwrap_or()` function
    ///
    /// returns `v` if `Ok()`; else returns `sub`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
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
            Err(_) => sub,
        }
    }

    /// Drop-in for the `unwrap_or_default()` function
    ///
    /// returns `v` if `Ok()`; else returns default value for type `T`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    ///     let mut foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.eval_or_default(), 7);
    ///     foo = Err("error");
    ///     assert_eq!(foo.eval_or_default(), 0);
    /// # }
    /// ```
    fn eval_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Ok(v) => v,
            Err(_) => Default::default(),
        }
    }

    /// Drop-in for the `unwrap_or_else()` function
    ///
    /// returns `v` if `Ok()`; else returns return value of function `func`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    ///     let mut foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.eval_or_else(|d| d.len() as u16), 7);
    ///
    ///     foo = Err("error");
    ///     assert_eq!(foo.eval_or_else(|d| d.len() as u16), 5);
    /// # }
    /// ```
    fn eval_or_else<F>(self, func: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Ok(v) => v,
            Err(e) => func(e),
        }
    }

    /// Drop-in for the `expect()` function
    ///
    ///## ⚠WARNING⚠
    /// Currently, the `should()` function calls the `errors!` macro instead of the `terrors!` macro on a failure. See the warnings on each of those macros for more information.
    ///
    /// This may cause unexpected behaviour, so be careful using it in threads
    ///
    /// returns `v` if `Ok()`; else errors with `panic` message and `Err()` value where `panic` implements std::fmt::Display
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    ///     let foo: Result<u16, &str> = Ok(7);
    ///     assert_eq!(foo.should("Should be set in initializer"), 7);
    /// # }
    /// ```
    fn should<M>(self, panic: M) -> T
    where
        M: Display,
    {
        match self {
            Ok(v) => v,
            Err(e) => errors!("{}. Error: {:?}", panic, e),
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

    #[test]
    fn test_eval_err() {
        let foo: Result<u16, &str> = Ok(7);
        assert_eq!(foo.should("Should be set in initializer"), 7);
    }
}
