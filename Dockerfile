# React build image
FROM node:18.0.0-alpine3.15 AS client_builder

WORKDIR /client
COPY ./front/package*.json .
RUN yarn install

ADD ./front .
RUN yarn build

# Actix-web build image
FROM ekidd/rust-musl-builder:stable as server_builder

WORKDIR /home/rust
COPY ./server .
RUN cargo build --release

FROM alpine:3.15 AS deploy

# Actix web's expose port
EXPOSE 8080
RUN apk --no-cache add postgresql-client
WORKDIR /app
COPY --from=client_builder /client/build ./build
COPY --from=server_builder /home/rust/target/x86_64-unknown-linux-musl/release/mendan ./
CMD [ "./mendan" ]
