FROM rust:1.71.1-slim-bookworm

RUN apt-get update
RUN apt-get -qq install build-essential

RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/findings

COPY . .

CMD ["dx", "serve"]
