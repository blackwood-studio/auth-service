/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use actix_web::web::Data;

use bcrypt::DEFAULT_COST;
use bcrypt::hash;

use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;

use uuid::Uuid;

use crate::entity::AccountEntity;

pub struct AccountService;

impl AccountService {
    pub async fn find_by_email(pool: &Data<PgPool>, email: &String) -> crate::Result<Option<AccountEntity>> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE email = $1;"#)
            .bind(email)
            .fetch_optional(&***pool)
            .await?
        )
    }

    pub async fn find_by_write_key(pool: &Data<PgPool>, write_key: &String) -> crate::Result<Option<AccountEntity>> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE write_key = $1;"#)
            .bind(write_key)
            .fetch_optional(&***pool)
            .await?
        )
    }

    pub async fn find_by_read_key(pool: &Data<PgPool>, read_key: &String) -> crate::Result<Option<AccountEntity>> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE read_key = $1;"#)
            .bind(read_key)
            .fetch_optional(&***pool)
            .await?
        )
    }

    pub async fn create(pool: &Data<PgPool>, email: &String, password: &String) -> crate::Result<AccountEntity> {
        let password_hash = hash(password, DEFAULT_COST)?;
        let write_key = Uuid::new_v4().to_string();
        let read_key = Uuid::new_v4().to_string();
        
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"INSERT INTO account(email, password_hash, write_key, read_key) VALUES ($1,$2,$3,$4) RETURNING id, email, password_hash, write_key, read_key;"#)
            .bind(email)
            .bind(password_hash)
            .bind(write_key)
            .bind(read_key)
            .fetch_one(&***pool)
            .await?
        )
    }

    pub async fn update(pool: &Data<PgPool>, write_key: &String, email: &String, password: &String) -> crate::Result<PgQueryResult> {
        let password_hash = hash(password, DEFAULT_COST)?;
        
        Ok(
            sqlx::query(r#"UPDATE account SET email = $1, password_hash = $2 WHERE write_key = $3;"#)
            .bind(email)
            .bind(password_hash)
            .bind(write_key)
            .execute(&***pool).await?
        )
    }

    pub async fn delete(pool: &Data<PgPool>, write_key: &String) -> crate::Result<PgQueryResult> {
        Ok(
            sqlx::query(r#"DELETE FROM account WHERE write_key = $1;"#)
            .bind(write_key)
            .execute(&***pool).await?
        )
    }
}
