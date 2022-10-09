//!# Info Utils
//!
//!## Utilities for displaying info in a pleasant manner
//!
//!## Features:
//!- `log!`, `warn!` and `error!` macros
//!- `eval()` as a drop-in method for `unwrap()`

pub mod output;
pub mod eval;

/// Import all features
pub mod prelude {
    pub use crate::{log, error, warn};

    pub use crate::eval::eval_option::EvalOption;
    pub use crate::eval::eval_result::EvalResult;
}