FROM rust:1.74

WORKDIR /app
COPY . /app

RUN cargo build

CMD ["/app/target/debug/auth-service"]
