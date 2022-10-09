//!# Eval
//!
//! Includes traits for `eval()` on `Option<T>` and `Result<T, E>` types
pub mod eval_result;
pub mod eval_option;

/// Import all `eval()` features
pub mod prelude {
    pub use crate::eval::eval_option::EvalOption;
    pub use crate::eval::eval_result::EvalResult;
}