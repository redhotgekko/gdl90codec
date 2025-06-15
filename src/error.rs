//! Maps [deku::error::DekuError] errors and other errors.
use deku::DekuError;
use std::{borrow::Cow, io::ErrorKind};

#[derive(Debug)]
pub enum GDL90Error {
    /// Parsing error when reading. See [deku::error::DekuError]
    Incomplete(usize),
    /// Parsing error when reading. See [deku::error::DekuError]
    Parse(Cow<'static, str>),
    /// Invalid parameter. See [deku::error::DekuError]
    InvalidParam(Cow<'static, str>),
    /// Assertion error from `assert` or `assert_eq` attributes. See [deku::error::DekuError]
    Assertion(Cow<'static, str>),
    /// Assertion error from `assert` or `assert_eq` attributes, without string. See [deku::error::DekuError]
    AssertionNoStr,
    /// Could not resolve `id` for variant. See [deku::error::DekuError]
    IdVariantNotFound,
    /// IO error while reading or writing. See [deku::error::DekuError]
    Io(ErrorKind),
    // ================== Codec specific errors ==================
    /// Unknown error to account for [deku::error::DekuError] being 'non_exhaustive'
    UnknownError(Cow<'static, str>),
    /// Tried to decode without data
    EmptyData,
    /// Incorrectly formatted message
    IncorrectlyFormatted,
    /// Checksum mismatch - actual vs expected
    ChecksumMismatch(u16, u16),
}

impl From<DekuError> for GDL90Error {
    fn from(value: DekuError) -> Self {
        match value {
            DekuError::Incomplete(need_size) => GDL90Error::Incomplete(need_size.bit_size()),
            DekuError::Parse(cow) => GDL90Error::Parse(cow),
            DekuError::InvalidParam(cow) => GDL90Error::InvalidParam(cow),
            DekuError::Assertion(cow) => GDL90Error::Assertion(cow),
            DekuError::AssertionNoStr => GDL90Error::AssertionNoStr,
            DekuError::IdVariantNotFound => GDL90Error::IdVariantNotFound,
            DekuError::Io(error_kind) => GDL90Error::Io(error_kind),
            err => GDL90Error::UnknownError(format!("{:?}", err).into()),
        }
    }
}
