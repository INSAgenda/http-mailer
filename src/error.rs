use std::io::Cursor;
use tiny_http::{Response, StatusCode};

/// Error returned by the HTTP API
#[allow(clippy::enum_variant_names)]
pub enum Error {
    AddressError(lettre::address::AddressError),
    LettreError(lettre::error::Error),
    SmtpError(lettre::transport::smtp::Error),
    MissingTo,
    MissingFrom,
    MissingSubject,
    MissingApiKey,
    Unauthorized(String),
}

impl From<lettre::address::AddressError> for Error {
    fn from(err: lettre::address::AddressError) -> Error {
        Error::AddressError(err)
    }
}

impl From<lettre::error::Error> for Error {
    fn from(err: lettre::error::Error) -> Error {
        Error::LettreError(err)
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(err: lettre::transport::smtp::Error) -> Error {
        Error::SmtpError(err)
    }
}

impl Error {
    pub fn description(&self) -> String {
        match self {
            Error::AddressError(err) => format!("Failed to parse address: {err}"),
            Error::MissingTo => String::from("Missing 'To' header"),
            Error::MissingFrom => String::from("Missing 'From' header"),
            Error::MissingSubject => String::from("Missing 'Subject' header"),
            Error::MissingApiKey => String::from("Missing 'ApiKey' header"),
            Error::LettreError(err) => format!("Lettre error: {err}"),
            Error::SmtpError(err) => format!("SMTP error: {err}"),
            Error::Unauthorized(api_key) => format!("Unauthorized api key: {api_key}"),
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Error::AddressError(_) => 400,
            Error::MissingTo | Error::MissingFrom | Error::MissingSubject => 400,
            Error::LettreError(_) => 500,
            Error::SmtpError(_) => 500,
            Error::Unauthorized(_) | Error::MissingApiKey => 401,
        }
    }
}

impl From<Error> for Response<Cursor<String>> {
    fn from(val: Error) -> Self {
        let description = val.description();
        let description_len = description.len();
        Response::new_empty(StatusCode(val.status_code()))
            .with_data(Cursor::new(description), Some(description_len))
    }
}
