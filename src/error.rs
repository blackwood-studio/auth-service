/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 * 
 * This file is part of the Auth Project.
 * 
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use actix_web::ResponseError;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum Error {
    ValidationError (validator::ValidationErrors),

    #[display(fmt = "A database request has failed")]
    DatabaseError,
    
    #[display(fmt = "During the hashing process something went wrong")]
    HashingError
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::ValidationError(errors) => HttpResponse::build(self.status_code()).body(errors.to_string()),
            _ => HttpResponse::build(self.status_code()).body(self.to_string())
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(errors: validator::ValidationErrors) -> Error {
        Error::ValidationError(errors)
    }
}

impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Error {
        Error::DatabaseError
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(_: bcrypt::BcryptError) -> Error {
        Error::HashingError
    }
}
