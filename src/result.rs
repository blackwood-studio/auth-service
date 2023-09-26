/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 * 
 * This file is part of the Auth Project.
 * 
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

use std::result;
use crate::exception::Exception;

pub type Result<T> = result::Result<T, Exception>;
