FROM rust:1.56 as builder

WORKDIR /app

RUN cargo new pokedex

COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./pokedex/Cargo.toml ./pokedex/

RUN cargo build --release --bin pokedex

COPY . .

RUN cargo build --release --bin pokedex

FROM ubuntu:focal

COPY --from=builder /app/target/release/pokedex /

CMD /pokedex
