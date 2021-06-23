FROM rust as rust_builder

WORKDIR /usr/src

RUN USER=root cargo new --bin priidik

WORKDIR /usr/src/priidik

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm ./target/release/deps/priidik*
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/priidik

RUN apt-get update
RUN apt-get install libopus0
RUN apt-get install -y --no-install-recommends ffmpeg

COPY --from=rust_builder /usr/src/priidik/target/release/priidik-bot ./priidik
COPY .env ./.env
COPY ./audio ./audio

CMD ./priidik