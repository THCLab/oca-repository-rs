FROM rust:1.65.0 as build

RUN USER=root cargo new --bin app
WORKDIR /app

RUN apt update && apt install lld clang -y
COPY Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/oca_repository*
RUN cargo build --release

FROM debian:10-slim
RUN apt update && apt install openssl -y
WORKDIR /app
COPY --from=build /app/target/release/oca-repository .
CMD ["./oca-repository"]
