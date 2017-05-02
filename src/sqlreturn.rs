/// Indicates the overall success or failure of the function
///
/// Each function in ODBC returns a code, known as its return code, which indicates the overall
/// success or failure of the function. Program logic is generally based on return codes.
/// See [ODBC reference](https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/return-codes-odbc)
#[allow(non_camel_case_types)]
#[repr(i16)]
#[must_use]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQLRETURN {
    /// No data left to read in column (see http://bit.ly/2p5KKD2)
    SQL_NO_DATA = -4,

    /// Function failed due to an invalid environment, connection, statement, or descriptor handle
    ///
    /// This indicates a programming error. No additional information is available from
    /// `SQLGetDiagRec` or `SQLGetDiagField`. This code is returned only when the handle is a null
    /// pointer or is the wrong type, such as when a statement handle is passed for an argument a
    /// connection handle.
    SQL_INVALID_HANDLE = -2,

    /// Function failed
    ///
    /// The application calls `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional
    /// information. The contents of any output arguments to the function are undefined.
    SQL_ERROR = -1,

    /// Function completed successfully
    ///
    /// The application calls `SQLGetDiagField` to retrieve additional information from the header
    /// record.
    SQL_SUCCESS = 0,

    /// Function completed successfully, possibly with a nonfatal error (warning)
    ///
    /// The application calls `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional
    /// information.
    SQL_SUCCESS_WITH_INFO = 1,

    /// A function that was started asynchronously is still executing
    ///
    /// The application `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional information if
    /// any.
    SQL_STILL_EXECUTING = 2,

    /// More data is needed
    ///
    /// ,such as when a parameter data is sent at execution time or additional connection
    /// information is required. The application calls `SQLGetDiagRec` or `SQLGetDiagField` to
    /// retrieve additional information, if any.
    SQL_NEED_DATA = 99,

    /// No more data was available
    ///
    /// The application calls `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional
    /// information. One or more driver-defined status records in class 02xxx may be returned.
    SQL_NO_DATA = 100,

    #[cfg(feature = "odbc_version_3_80")]
    SQL_PARAM_DATA_AVAILABLE = 101,
}

pub use self::SQLRETURN::*;
