# Auth-Service

## Requirements

- [WSL Ubuntu](https://apps.microsoft.com/detail/ubuntu/9PDXGNCFSCZV?hl=en-us&gl=US)
- [Rust](https://www.rust-lang.org/learn/get-started)
- [PostgreSQL](https://www.postgresql.org/download/linux/ubuntu/)
- [sqlx-cli](https://crates.io/crates/sqlx-cli)

## Setup for Development

1. Start your PostgreSQL service

``` bash
sudo service postgresql start
```

2. Connect to your service

``` bash
sudo -u postgres psql
```

3. Create a database named `test`

``` sql
CREATE DATABASE test;
```

4. Clone the [auth-service](https://github.com/blackwood-studio/auth-service) project

``` bash
git clone https://github.com/blackwood-studio/auth-service.git
```

5. Open a terminal in your auth-service directory and start the migration script

``` bash
sqlx-cli migrate run
```

6. Set the database credentials in the .env file
7. Run Cargo

``` bash
cargo run
```


## Setup for Production

1. Start your PostgreSQL service

``` bash
sudo service postgresql start
```

2. Connect to your service

``` bash
sudo -u postgres psql
```

3. Create a database named `production`

``` sql
CREATE DATABASE production;
```

4. Download the latest release of [auth-service](https://github.com/blackwood-studio/auth-service/releases) for your platform
5. Open a terminal in your auth-service directory and start the migration script

``` bash
sqlx-cli migrate run
```

6. Set the database credentials in the .env file
7. Start the auth-service

``` bash
auth-service
```
