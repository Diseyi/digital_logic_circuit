use std::error::Error;
use std::fmt;

// Custom error type
#[derive(Debug)]
pub struct InvalidValueError;

impl fmt::Display for InvalidValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value: a or b is not 0 or 1")
    }
}

impl Error for InvalidValueError {}

pub trait Logic {
    fn logic(&self) -> Result<u8, InvalidValueError>;
}

pub struct AND {
    pub a: u8,
    pub b: u8,
}
impl Logic for AND {
    fn logic(&self) -> Result<u8, InvalidValueError> {
        if (self.a != 0 && self.a != 1) || (self.b != 0 && self.b != 1) {
            return Err(InvalidValueError);
        }
        if self.a != 0 && self.b != 0 {
            Ok(1)
        } else {
            Ok(0)
        }
    }
}

// TODO
// impl Fn<()> for AND {
//     extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//         self.logic()
//     }
// }
