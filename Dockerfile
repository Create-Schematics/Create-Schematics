FROM rust:latest as build

WORKDIR /usr/src/createschematics

COPY . .

ARG SQLX_OFFLINE=true
RUN cargo build --release
