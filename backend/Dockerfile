FROM rust:1.87-slim-bookworm

WORKDIR /app

COPY Cargo.toml Cargo.lock /app/

COPY src /app/src

RUN cargo build --release

EXPOSE 8081

CMD [ "/app/target/release/backend" ]
