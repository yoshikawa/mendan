# React build image
FROM node:18.0.0-alpine3.15 AS client_builder

WORKDIR /client
COPY ./front/package*.json .
RUN yarn install

ADD ./front .
RUN yarn build

# Actix-web build image
FROM rust:1.60 AS server_builder

RUN apt-get update && apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /server
RUN USER=root cargo new mendan
WORKDIR /server/mendan
COPY ./server/Cargo.toml ./server/Cargo.lock ./
RUN cargo build --release
COPY ./server .
RUN rm ./target/release/deps/mendan*
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# FROM alpine:3.15 AS deploy

# Actix web's expose port
# EXPOSE 8080
# RUN apk --no-cache add postgresql-client
# WORKDIR /app
# COPY --from=client_builder /client/build ./build
# COPY --from=server_builder /server/mendan/target/x86_64-unknown-linux-musl/release/mendan ./
# CMD [ "./mendan" ]
