FROM rust:1.74

WORKDIR /app
COPY . /app

RUN cargo install sqlx-cli

CMD ["sqlx", "migrate", "run"]
