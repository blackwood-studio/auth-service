/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct Auth {
    #[serde(default)] 
    #[validate(email(message = "Invalid email address"))]
    #[validate(length(min = 1, message = "The min size of the email is 1"))]
    #[validate(length(max = 255, message = "The max size of the email is 255"))]
    pub email: String,

    #[serde(default)] 
    #[validate(length(min = 8, message = "The min size of the password is 8"))]
    #[validate(length(max = 255, message = "The max size of the password is 255"))]
    pub password: String
}
