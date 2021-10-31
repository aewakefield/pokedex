FROM ubuntu:focal as final

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

FROM rust:1.56 as builder

WORKDIR /app

RUN cargo new pokedex
RUN cargo new pokeapi

COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./pokedex/Cargo.toml ./pokedex/
COPY ./pokeapi/Cargo.toml ./pokeapi/

RUN cargo build --release --bin pokedex

COPY . .

RUN cargo build --release --bin pokedex

FROM final

COPY --from=builder /app/target/release/pokedex /

CMD /pokedex
