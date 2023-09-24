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

use derive_more::Display;
use derive_more::Error;

#[derive(Debug, Display, Error)]
pub enum Exception {
    ValidationError (validator::ValidationErrors),

    #[display(fmt = "A database request has failed")]
    DatabaseError,
    
    #[display(fmt = "During the hashing process something went wrong")]
    HashingError
}

impl ResponseError for Exception {
    fn error_response(&self) -> HttpResponse {
        match self {
            Exception::ValidationError(errors) => HttpResponse::build(self.status_code()).json(errors),
            _ => HttpResponse::build(self.status_code()).body(self.to_string())
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Exception::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<validator::ValidationErrors> for Exception {
    fn from(errors: validator::ValidationErrors) -> Self {
        Exception::ValidationError(errors)
    }
}

impl From<sqlx::Error> for Exception {
    fn from(_: sqlx::Error) -> Self {
        Exception::DatabaseError
    }
}

impl From<bcrypt::BcryptError> for Exception {
    fn from(_: bcrypt::BcryptError) -> Self {
        Exception::HashingError
    }
}
