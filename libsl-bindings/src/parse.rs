use core::num::IntErrorKind;
pub use libsl::parse::*; //re-export для uniffi-scaffolding
pub use std::num::ParseIntError;

uniffi::custom_type!(ParseIntError, FFIParseIntError, {
    remote,
    lower: |err| err.into(),
    try_lift: |_err| unimplemented!()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FFIParseIntError {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Zero,
    Unknown,
}

impl From<ParseIntError> for FFIParseIntError {
    fn from(value: ParseIntError) -> Self {
        match value.kind() {
            IntErrorKind::Empty => Self::Empty,
            IntErrorKind::InvalidDigit => Self::InvalidDigit,
            IntErrorKind::PosOverflow => Self::PosOverflow,
            IntErrorKind::NegOverflow => Self::NegOverflow,
            IntErrorKind::Zero => Self::Zero,
            _ => Self::Unknown,
        }
    }
}
