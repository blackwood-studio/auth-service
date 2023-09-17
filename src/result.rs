/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 * 
 * This file is part of the Auth Project.
 * 
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use std::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error>>;
