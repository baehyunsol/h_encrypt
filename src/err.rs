use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;

#[derive(Debug)]
pub enum EncError {
    DecryptionTooShort,
    FileNotFound,
    PermissionDenied,
    FileIOError(IOErrorKind)
}

impl From<IOError> for EncError {

    fn from(io_err: IOError) -> Self {

        match io_err.kind() {
            IOErrorKind::NotFound => EncError::FileNotFound,
            IOErrorKind::PermissionDenied => EncError::PermissionDenied,
            e => EncError::FileIOError(e)
        }

    }

}