use crate::error;

pub trait EvalOption<T> {
    fn eval(self) -> T;
    fn eval_or(self, sub: T) -> T;
    fn eval_or_default(self) -> T where T: Default;
    fn eval_or_else<F>(self, func: F) -> T where F: FnOnce() -> T;
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

    fn eval_or_default(self) -> T where T: Default {
        match self {
            Some(v) => v,
            None => Default::default(),
        }
    }

    fn eval_or_else<F>(self, func: F) -> T where F: FnOnce() -> T {
        match self {
            Some(v) => v,
            None => func(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        struct Foo {
            data: Option<i32>
        }

        let val: Foo = Foo {
            data: Some(7)
        };
        assert_eq!(val.data.eval(), 7);
    }

    #[test]
    fn test_eval_or() {
        struct Foo<'a> {
            data: Option<&'a str>
        }

        let mut val: Foo = Foo {
            data: Some("info")
        };

        assert_eq!(val.data.eval_or("data"), "info");

        val.data = None;
        assert_eq!(val.data.eval_or("data"), "data");
    }

    #[test]
    fn test_eval_or_default() {
        struct Foo {
            data: Option<i32>
        }

        let mut val: Foo = Foo {
            data: Some(7)
        };

        assert_eq!(val.data.eval_or_default(), 7);

        val.data = None;
        assert_eq!(val.data.eval_or_default(), 0);
    }

    #[test]
    fn test_eval_or_else() {
        struct Foo {
            data: Option<i32>
        }

        let mut val: Foo = Foo {
            data: Some(7)
        };

        let foo = 10;
        assert_eq!(val.data.unwrap_or_else(|| 3*foo), 7);

        val.data = None;
        assert_eq!(val.data.unwrap_or_else(|| 3*foo), 30);
    }
}