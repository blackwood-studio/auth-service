/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use actix_web::web::Data;
use sqlx::PgPool;
use crate::entity::AccountEntity;

pub struct AccountService;

impl AccountService {
    pub async fn find_by_email(pool: &Data<PgPool>, email: &String) -> crate::Result<Option<AccountEntity>> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE email = $1"#)
            .bind(email)
            .fetch_optional(&***pool)
            .await?
        )
    }

    pub async fn find_by_read_key(pool: &Data<PgPool>, read_key: &String) -> crate::Result<Option<AccountEntity>> {
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"SELECT * FROM account WHERE read_key = $1"#)
            .bind(read_key)
            .fetch_optional(&***pool)
            .await?
        )
    }

    pub async fn create(pool: Data<PgPool>, email: &String, password: &String) -> crate::Result<AccountEntity> {
        let entity = AccountEntity::new(email, password)?;
        
        Ok(
            sqlx::query_as::<_, AccountEntity>(r#"INSERT INTO account(email, password, write_key, read_key) VALUES ($1,$2,$3,$4) RETURNING id, email, password, write_key, read_key;"#)
            .bind(&entity.email)
            .bind(&entity.password)
            .bind(&entity.write_key)
            .bind(&entity.read_key)
            .fetch_one(&**pool)
            .await?
        )
    }
}