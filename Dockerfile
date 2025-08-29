FROM rust:1.89.0 AS builder
WORKDIR /usr/src/app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/src/app
RUN mkdir /data
COPY --from=builder /usr/src/app/target/release/RustyMQTTEventController ./
ENV RUST_BACKTRACE=full
CMD ["./RustyMQTTEventController"]