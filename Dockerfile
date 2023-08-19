FROM node:20-alpine3.17 AS tailwind

WORKDIR /usr/src/findings

COPY . .

RUN npx tailwindcss@3.3.3 -i ./input.css -o ./public/tailwind.css

FROM rust:1.71.1-slim-bookworm AS builder

RUN apt-get update
RUN apt-get -qq install build-essential libssl-dev pkg-config

RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown

COPY --from=tailwind /usr/src/findings /usr/src/findings

WORKDIR /usr/src/findings

RUN dx build --release --features web
RUN dx build --release --features ssr --platform desktop

CMD ["./dist/findings"]
