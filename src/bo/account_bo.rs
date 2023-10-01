/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use serde::Serialize;
use crate::entity::AccountEntity;

#[derive(Serialize)]
pub struct AccountBo {
    pub id: i32,
    pub email: String
}

impl From<AccountEntity> for AccountBo {
    fn from(entity: AccountEntity) -> AccountBo {
        AccountBo { 
            id: entity.id, 
            email: entity.email
        }
    }
}
