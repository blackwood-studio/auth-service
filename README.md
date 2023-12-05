![Blackwood Studio Authentication Logo](https://github.com/blackwood-studio/auth-service/assets/74761380/824dd5ce-011a-427f-adcc-e8c35616d52a)

## Requirements for development

- [PostgreSQL](https://www.postgresql.org/download/)
- [Rust](https://www.rust-lang.org/learn/get-started/)
- [sqlx-cli](https://crates.io/crates/sqlx-cli/)

## Setup for development

1. Create .env file like the following example
``` env
# auth-service
HOST_ADDRESS           = 'localhost'
DATABASE_NAME          = 'test'
DATABASE_USER_NAME     = 'postgres'
DATABASE_USER_PASSWORD = '123456'

# sqlx-cli
DATABASE_URL = 'postgres://postgres:123456@localhost/test'
```

2. Create a postgresql database based on the .env file

3. Execute all migration scripts using sqlx-cli

``` bash
sqlx migrate run
```

4. Start the auth-service

``` bash
cargo run
```

## Requirements for production

- [Docker](https://www.docker.com/get-started/)

## Setup for production

1. Create .env file like the following example
``` env
# auth-service
HOST_ADDRESS           = 'postgresql'
DATABASE_NAME          = 'test'
DATABASE_USER_NAME     = 'postgres'
DATABASE_USER_PASSWORD = '123456'

# sqlx-cli
DATABASE_URL = 'postgres://postgres:123456@postgresql:5432/test'
```

2. Execute a docker build

``` bash
docker compose build
```

3. Start your containers

``` bash
docker compose up
```
