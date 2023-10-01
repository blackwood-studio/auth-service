/*******************************************************
 * Copyright (C) 2023 - present, Blackwood Studio
 *
 * This file is part of the Auth Project.
 *
 * The Auth Project can not be copied and/or distributed without the express
 * permission of an Blackwood Studio Admin
 *******************************************************/

CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    write_key VARCHAR(255) NOT NULL,
    read_key VARCHAR(255) NOT NULL
);
