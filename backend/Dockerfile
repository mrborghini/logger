ARG APP_NAME=logger-backend

FROM rust:alpine AS builder

ARG APP_NAME
WORKDIR /usr/src

RUN apk add --no-cache musl-dev

COPY . .

COPY .env.prod .env

RUN cargo install --path .

FROM alpine:latest

# Copy the executable from the "build" stage.
ARG APP_NAME
WORKDIR /usr/app
COPY --from=builder /usr/local/cargo/bin/$APP_NAME /usr/local/bin/app
COPY --from=builder /usr/src/.env .

EXPOSE 3000

ENTRYPOINT ["app"]