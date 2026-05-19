mod codec;
mod error;
mod program;
mod util;

pub use codec::BytecodeFile;
pub use error::BytecodeError;
pub use program::Program;

use util::{read_i64, read_u16, read_u64_as_usize};
