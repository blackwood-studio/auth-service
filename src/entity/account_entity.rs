/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use bcrypt::DEFAULT_COST;
use bcrypt::BcryptError;
use bcrypt::hash;

use bcrypt::verify;
use sqlx::FromRow;

use uuid::Uuid;

#[derive(FromRow)]
pub struct AccountEntity {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub write_key: String,
    pub read_key: String
}

impl AccountEntity {
    pub fn new(email: &String, password: &String) -> Result<AccountEntity, BcryptError> {
        Ok(
            AccountEntity { 
                id: 0, 
                email: email.clone(),
                password: hash(password, DEFAULT_COST)?,
                write_key: Uuid::new_v4().to_string(),
                read_key: Uuid::new_v4().to_string()
            }
        )
    }

    pub fn verify(&self, password: &String) -> Result<bool, BcryptError> {
        Ok(
            verify(password, &self.password)?
        )
    }
}
