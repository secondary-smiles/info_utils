//!# Info Utils
//!
//!## Utilities for displaying info in a pleasant manner
//!
//!## Features:
//!- `log!`, `warn!` and `error!` macros
//!- `eval()` as a drop-in method for `unwrap()`

pub mod eval;
pub mod output;

/// Import all features
pub mod prelude {
    pub use crate::{error, log, warn};
    pub use crate::{errors, logs, warns};

    pub use crate::eval::eval_option::EvalOption;
    pub use crate::eval::eval_result::EvalResult;
}

/// Import only logging macros
pub mod macros {
    pub use crate::{error, log, warn};
    pub use crate::{errors, logs, warns};
}
