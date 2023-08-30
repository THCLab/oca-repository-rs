FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN     apt-get update --quiet \
        && apt-get install -y libclang-dev
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN     apt-get update --quiet \
        && apt-get install -y tini openssl sqlite3
COPY --from=builder /app/target/release/oca-repository /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["tini", "--"]
CMD /usr/local/bin/oca-repository
