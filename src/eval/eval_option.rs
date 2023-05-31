//!# Eval Option
//!
//! `eval()` implemented for the `Option<T>` type

use crate::errors;

/// Eval Option trait that enables `eval()` for the `Option<T>` type
pub trait EvalOption<T> {
    fn eval(self) -> T;
    fn eval_or(self, sub: T) -> T;
    fn eval_or_default(self) -> T
    where
        T: Default;
    fn eval_or_else<F>(self, func: F) -> T
    where
        F: FnOnce() -> T;
    fn should<M>(self, panic: M) -> T
    where
        M: std::fmt::Display;
}

impl<T> EvalOption<T> for Option<T> {
    /// Drop-in for the `unwrap()` function
    ///
    /// returns `v` if `Some`; else calls an `errors!`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    /// struct Foo {
    ///      data: Option<i32>,
    ///  }
    ///
    ///  let val: Foo = Foo {
    ///      data: Some(7)
    ///  };
    ///  assert_eq!(val.data.eval(), 7);
    /// # }
    /// ```
    fn eval(self) -> T {
        match self {
            Some(v) => v,
            None => errors!("called `Option::eval()` on a `None` value"),
        }
    }

    /// Drop-in for the `unwrap_or()` function
    ///
    /// returns `v` if `Some`; else returns `sub`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    ///  struct Foo<'a> {
    ///     data: Option<&'a str>,
    ///  }
    ///
    ///  let mut val: Foo = Foo {
    ///     data: Some("info")
    ///  };
    ///
    ///  assert_eq!(val.data.eval_or("data"), "info");
    ///
    ///  val.data = None;
    ///  assert_eq!(val.data.eval_or("data"), "data");
    /// # }
    /// ```
    fn eval_or(self, sub: T) -> T {
        match self {
            Some(v) => v,
            None => sub,
        }
    }

    /// Drop-in for the `unwrap_or_default()` function
    ///
    /// returns `v` if `Some`; else returns default value for type `T`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    /// struct Foo {
    ///    data: Option<i32>,
    /// }
    ///
    /// let mut val: Foo = Foo {
    /// data: Some(7)
    /// };
    ///
    /// assert_eq!(val.data.eval_or_default(), 7);
    ///
    /// val.data = None;
    /// assert_eq!(val.data.eval_or_default(), 0);
    /// # }
    /// ```
    fn eval_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Some(v) => v,
            None => Default::default(),
        }
    }

    /// Drop-in for the `unwrap_or_else()` function
    ///
    /// returns `v` if `Some`; else returns return value of function `func`
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    /// struct Foo {
    ///     data: Option<i32>,
    /// }
    ///
    /// let mut val: Foo = Foo {
    ///     data: Some(7)
    /// };
    ///
    /// let bar = 10;
    /// assert_eq!(val.data.eval_or_else(|| 3 * bar), 7);
    ///
    /// val.data = None;
    /// assert_eq!(val.data.eval_or_else(|| 3 * bar), 30);
    /// # }
    /// ```
    fn eval_or_else<F>(self, func: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(v) => v,
            None => func(),
        }
    }

    /// Drop-in for the `expect()` function
    ///
    ///## ⚠WARNING⚠
    /// Currently, the `should()` function calls the `errors!` macro instead of the `terrors!` macro on a failure. See the warnings on each of those macros for more information.
    ///
    /// This may cause unexpected behaviour, so be careful using it in threads
    ///
    /// returns `v` if `Some`; else errors with `panic` message where `panic` implements std::fmt::Display
    ///
    /// Example
    /// ```rust
    /// # use crate::info_utils::prelude::*;
    /// # fn main() {
    /// struct Foo {
    ///     data: Option<i32>,
    /// }
    ///
    /// let mut val: Foo = Foo {
    ///     data: Some(7)
    /// };
    ///
    /// assert_eq!(val.data.should("Should be set in initializer"), 7);
    ///
    /// /* Errors with message "Should be set in initializer":
    /// val.data = None;
    /// val.data.eval_err("Should be set in initializer");
    /// */
    /// # }
    /// ```
    fn should<M>(self, panic: M) -> T
    where
        M: std::fmt::Display,
    {
        match self {
            Some(v) => v,
            None => errors!("{}", panic),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        struct Foo {
            data: Option<i32>,
        }

        let val: Foo = Foo { data: Some(7) };
        assert_eq!(val.data.eval(), 7);
    }

    #[test]
    fn test_eval_or() {
        struct Foo<'a> {
            data: Option<&'a str>,
        }

        let mut val: Foo = Foo { data: Some("info") };

        assert_eq!(val.data.eval_or("data"), "info");

        val.data = None;
        assert_eq!(val.data.eval_or("data"), "data");
    }

    #[test]
    fn test_eval_or_default() {
        struct Foo {
            data: Option<i32>,
        }

        let mut val: Foo = Foo { data: Some(7) };

        assert_eq!(val.data.eval_or_default(), 7);

        val.data = None;
        assert_eq!(val.data.eval_or_default(), 0);
    }

    #[test]
    fn test_eval_or_else() {
        struct Foo {
            data: Option<i32>,
        }

        let mut val: Foo = Foo { data: Some(7) };

        let bar = 10;
        assert_eq!(val.data.eval_or_else(|| 3 * bar), 7);

        val.data = None;
        assert_eq!(val.data.eval_or_else(|| 3 * bar), 30);
    }

    #[test]
    fn test_should() {
        struct Foo {
            data: Option<i32>,
        }

        let val: Foo = Foo { data: Some(7) };
        assert_eq!(val.data.should("Data set to Some by initialization"), 7);

        // Also works with other data types that implement std::fmt::Display
        let val: Foo = Foo { data: Some(7) };
        assert_eq!(val.data.should(7 as f32 * 3.7), 7);
    }
}
