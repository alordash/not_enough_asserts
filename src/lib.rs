pub mod panics;
pub mod r#type;

pub mod prelude {
    pub use crate::assert_type_eq;
    pub use crate::panics::*;
}
