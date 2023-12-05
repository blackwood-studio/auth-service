![Blackwood Studio Authentication Logo](https://github.com/blackwood-studio/auth-service/assets/74761380/824dd5ce-011a-427f-adcc-e8c35616d52a)

## Requirements

- [Docker](https://www.docker.com/get-started/)

## Setup for development

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
docker compose --profile development build
```

3. Start your containers

``` bash
docker compose --profile development up
```

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
docker compose --profile production build
```

3. Start your containers

``` bash
docker compose --profile production up
```
