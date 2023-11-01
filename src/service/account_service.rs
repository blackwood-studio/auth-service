/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use bcrypt::DEFAULT_COST;
use bcrypt::hash;

use sqlx::Postgres;
use sqlx::Transaction;

use uuid::Uuid;

use crate::Error;
use crate::entity::AccountEntity; 

pub struct AccountService;

impl AccountService {
    pub async fn find_by_email(transaction: &mut Transaction<'_, Postgres>, email: &String) -> Result<Option<AccountEntity>, Error> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE email = $1;"#)
            .bind(email)
            .fetch_optional(&mut **transaction)
            .await?
        )
    }

    pub async fn find_by_write_key(transaction: &mut Transaction<'_, Postgres>, write_key: &String) -> Result<Option<AccountEntity>, Error> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE write_key = $1;"#)
            .bind(write_key)
            .fetch_optional(&mut **transaction)
            .await?
        )
    }

    pub async fn find_by_read_key(transaction: &mut Transaction<'_, Postgres>, read_key: &String) -> Result<Option<AccountEntity>, Error> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE read_key = $1;"#)
            .bind(read_key)
            .fetch_optional(&mut **transaction)
            .await?
        )
    }

    pub async fn create(transaction: &mut Transaction<'_, Postgres>, email: &String, password: &String) -> Result<AccountEntity, Error> {
        let password_hash = hash(password, DEFAULT_COST)?;
        let write_key = Uuid::new_v4().to_string();
        let read_key = Uuid::new_v4().to_string();
        
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"INSERT INTO account(email, password_hash, write_key, read_key) VALUES ($1,$2,$3,$4) RETURNING id, email, password_hash, write_key, read_key;"#)
            .bind(email)
            .bind(password_hash)
            .bind(write_key)
            .bind(read_key)
            .fetch_one(&mut **transaction)
            .await?
        )
    }

    pub async fn update(transaction: &mut Transaction<'_, Postgres>, write_key: &String, email: &Option<String>, password: &Option<String>) -> Result<(), Error> {
        if let Some(email) = email {
            sqlx::query(r#"UPDATE account SET email = $1 WHERE write_key = $2;"#)
            .bind(email)
            .bind(write_key)
            .execute(&mut **transaction)
            .await?;
        }

        if let Some(password) = password {
            let password_hash = hash(password, DEFAULT_COST)?;

            sqlx::query(r#"UPDATE account SET password_hash = $1 WHERE write_key = $2;"#)
            .bind(password_hash)
            .bind(write_key)
            .execute(&mut **transaction)
            .await?;
        }

        Ok(())
    }

    pub async fn delete(transaction: &mut Transaction<'_, Postgres>, write_key: &String) -> Result<(), Error> {
        sqlx::query(r#"DELETE FROM account WHERE write_key = $1;"#)
        .bind(write_key)
        .execute(&mut **transaction)
        .await?;

        Ok(())
    }
}
