/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use sqlx::FromRow;

#[derive(FromRow)]
pub struct AccountEntity {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub write_key: String,
    pub read_key: String
}
