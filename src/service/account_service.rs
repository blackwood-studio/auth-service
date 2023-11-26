/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use dotenv_codegen::dotenv;

use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgConnectOptions;

use crate::Error;
use crate::transaction::AccountTransaction;

pub struct AccountService {
    pool: Pool<Postgres>
}

impl AccountService {
    pub fn new() -> AccountService {
        let options = PgConnectOptions::new()
        .host(dotenv!("HOST_ADDRESS"))
        .database(dotenv!("DATABASE_NAME"))
        .username(dotenv!("DATABASE_USER_NAME"))
        .password(dotenv!("DATABASE_USER_PASSWORD"));

        AccountService { pool: PgPool::connect_lazy_with(options) }
    }

    pub async fn transaction<'t>(&'t self) -> Result<AccountTransaction<'t>, Error> {
        let transaction = self.pool.begin().await?;
        
        Ok(
            AccountTransaction::new(transaction)
        )
    }
}
