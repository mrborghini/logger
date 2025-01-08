ARG APP_NAME=logger-backend

FROM rust:alpine AS builder

ARG APP_NAME
WORKDIR /usr/src/$APP_NAME

RUN apk add --no-cache musl-dev

COPY . .

COPY .env.prod .env

RUN cargo install --path .

RUN mv /usr/local/cargo/bin/$APP_NAME /usr/local/cargo/bin/server

FROM alpine:latest

# Copy the executable from the "build" stage.
ARG APP_NAME
WORKDIR /usr/app
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/src/$APP_NAME/.env .

EXPOSE 3000

ENTRYPOINT ["server"]