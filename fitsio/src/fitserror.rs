use std::result;
use std::fmt;
use stringutils::status_to_string;

/// Error type
///
/// `cfitsio` passes errors through integer status codes. This struct wraps this and its associated
/// error message.
#[derive(Debug, PartialEq, Eq)]
pub struct FitsError {
    pub status: i32,
    pub message: String,
}

/// Display implementation for `FitsError`
///
/// This enables the error to be printed in a user-facing way
impl fmt::Display for FitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let full_message = format!("Fits status {}: {}", self.status, self.message);
        full_message.fmt(f)
    }
}

/// Error implementation for `FitsError`
///
/// This enables the fits error type to be treated as a Box<Error>
impl ::std::error::Error for FitsError {
    fn description(&self) -> &str {
        "fits error"
    }
}

/// Allowing any custom error messages to be turned into a `FitsError`
///
/// For example:
///
/// ```rust
/// # use fitsio::fitserror::FitsResult;
/// let result: FitsResult<()> = Err("error messsage".into());
/// if let Err(e) = result {
///     assert_eq!(e.status, 600);
/// }
/// ```
impl<'a> From<&'a str> for FitsError {
    fn from(s: &'a str) -> Self {
        FitsError {
            status: 600,
            message: s.to_string(),
        }
    }
}

/// Macro for returning a FITS error type
macro_rules! fits_try {
    ($status: ident, $e: expr) => {
        match $status {
            0 => Ok($e),
            _ => {
                Err(FitsError {
                    status: $status,
                    // unwrap guaranteed to work as we know $status > 0
                    message: stringutils::status_to_string($status).unwrap(),
                })
            }
        }
    }
}

/// FITS specific result type
///
/// This is a shortcut for a result with `FitsError` as the error type
pub type FitsResult<T> = result::Result<T, FitsError>;

pub fn status_to_error(status: i32) -> FitsResult<()> {
    Err(FitsError {
        status: status,
        // unwrap guaranteed to work as we know status > 0
        message: status_to_string(status).unwrap(),
    })
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use super::FitsError;

    #[test]
    fn fits_error_implements_error() {
        fn foo() -> ::std::result::Result<(), Box<Error>> {
            let err = FitsError {
                status: 100,
                message: "test".into(),
            };
            Err(err.into())
        }

        if let Err(e) = foo() {
            assert_eq!(format!("{}", e), "Fits status 100: test".to_string());
            assert_eq!(e.description(), "fits error");
        }
    }
}
