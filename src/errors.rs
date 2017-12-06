use finchers::{Response, Responder};
use finchers::request::ParseBodyError;
use finchers::util::NoReturn;
use hyper::StatusCode;
use serde_json;

#[derive(Debug)]
pub enum ApiError {
    ParseBody(ParseBodyError<serde_json::error::Error>),
}

impl From<ParseBodyError<serde_json::error::Error>> for ApiError {
    fn from(err: ParseBodyError<serde_json::error::Error>) -> Self {
        ApiError::ParseBody(err)
    }
}

impl Responder for ApiError {
    type Error = NoReturn;
    fn respond(self) -> Result<Response, Self::Error> {
        use ApiError::*;
        match self {
            ParseBody(..) => Ok(Response::new().with_status(StatusCode::BadRequest)),
        }
    }
}
