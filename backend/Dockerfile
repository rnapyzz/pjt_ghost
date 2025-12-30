FROM rust:latest

WORKDIR /app

RUN cargo install cargo-watch

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm -rf src

COPY . .

CMD ["cargo", "watch", "-x", "run"]