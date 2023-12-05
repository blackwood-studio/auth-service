FROM rust:1.74

WORKDIR /app
COPY . /app

RUN cargo build --release

CMD ["/app/target/release/auth-service"]
