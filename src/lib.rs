pub mod output;
pub mod result;

pub mod prelude {
    pub use crate::{log, error, warn};

    pub use crate::result::eval_option::EvalOption;
    pub use crate::result::eval_result::EvalResult;
}