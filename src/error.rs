use std::fmt;
use std::result::Result as StdResult;

pub(crate) type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub struct Error {
    name: String,
    description: String,
    cause: Option<String>,
}

impl Error {
    fn new<E: fmt::Debug>(name: &str, description: &str, cause: E) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            cause: Some(format!("{:?}", cause)),
        }
    }

    pub(crate) fn new_custom(name: &str, description: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            cause: None,
        }
    }

    pub fn show_error(self) {
        error!("{}", self);
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in {}: {}", self.name, self.description)?;

        if let Some(ref cause) = self.cause {
            write!(f, " ({})", cause)?;
        }

        Ok(())
    }
}

pub(crate) trait WrapErrorExt<T> {
    fn wrap_error(self, name: &str, description: &str) -> Result<T>;
}

impl<T, E: fmt::Debug> WrapErrorExt<T> for StdResult<T, E> {
    fn wrap_error(self, name: &str, description: &str) -> Result<T> {
        self.map_err(|error| Error::new(name, description, error))
    }
}

impl<T> WrapErrorExt<T> for Option<T> {
    fn wrap_error(self, name: &str, description: &str) -> Result<T> {
        self.ok_or_else(|| Error::new_custom(name, description))
    }
}

pub(crate) trait ResultExt<T> {
    fn show_error(self) -> StdResult<T, ()>;
}

impl<T> ResultExt<T> for Result<T> {
    fn show_error(self) -> StdResult<T, ()> {
        self.map_err(|error| error.show_error())
    }
}
