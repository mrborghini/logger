ARG APP_NAME=logger-backend

FROM rust AS builder

ARG APP_NAME
WORKDIR /usr/src/$APP_NAME

COPY . .

RUN cargo install --path .

RUN mv /usr/local/cargo/bin/$APP_NAME /usr/local/cargo/bin/server

FROM debian:latest

# Copy the executable from the "build" stage.
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

# Add .env to root
COPY .env .

EXPOSE 8080

CMD ["server"]