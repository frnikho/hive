FROM rust:1.71.1-buster as builder

RUN USER=root cargo new --bin api
WORKDIR /api

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm -rf src

COPY ./src ./src

RUN cargo build --release


FROM rust:1.71.1-slim-buster

COPY --from=builder /api/target/release/api .

CMD ["./api"]