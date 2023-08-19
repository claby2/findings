FROM node:20-alpine3.17 AS tailwind

WORKDIR /usr/src/findings

COPY . .

RUN npx tailwindcss@3.3.3 -i ./input.css -o ./public/tailwind.css

FROM rust:1.71.1-slim-bookworm AS builder

RUN apt-get update
RUN apt-get -qq install build-essential

RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown

COPY --from=tailwind /usr/src/findings /usr/src/findings

WORKDIR /usr/src/findings

RUN dx build --release

FROM nginx:1.25.2-alpine3.18

COPY --from=builder /usr/src/findings/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /usr/src/findings/dist /usr/share/nginx/html
