#![doc = include_str!("../README.md")]
#![forbid(
    unsafe_code,
    unused_imports,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    overflowing_literals
)]
#![warn(missing_docs)]

mod splash_screen;

///
pub mod prelude {
    pub use crate::splash_screen::state::*;
    pub use crate::splash_screen::tick::*;
    pub use crate::splash_screen::*;

    pub use crate::splash_screen::ui::*;
}
