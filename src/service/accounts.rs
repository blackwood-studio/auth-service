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

use crate::model::Account;

pub struct Accounts;

impl Accounts {
    pub async fn find_by_email(pool: &Data<PgPool>, email: &String) -> crate::Result<Option<Account>> {
        Ok(
            sqlx::query_as::<_, Account>(r#"SELECT * FROM account WHERE email = $1"#)
            .bind(email)
            .fetch_optional(&***pool)
            .await?
        )
    }

    pub async fn create(pool: Data<PgPool>, email: &String, password: &String) -> crate::Result<Account> {
        let account = Account::new(email, password)?;
        
        Ok(
            sqlx::query_as::<_, Account>(r#"INSERT INTO account(email, password, write_key, read_key) VALUES ($1,$2,$3,$4) RETURNING id, email, password, write_key, read_key;"#)
            .bind(&account.email)
            .bind(&account.password)
            .bind(&account.write_key)
            .bind(&account.read_key)
            .fetch_one(&**pool)
            .await?
        )
    }
}
