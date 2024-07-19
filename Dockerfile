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
RUN cargo build --release --features data_entries_xls

FROM debian:bookworm-slim AS runtime
WORKDIR /app
LABEL org.opencontainers.image.description "The OCA repository is a key concept of the OCA Ecosystem. It enables the management, storage, and sharing of OCA Objects like OCA Bundles, Capture Bases, and Overlays. Furthermore, it comes with pre-baked support for OCAFiles. The interface is exposed through REST API"
RUN     apt-get update --quiet \
        && apt-get install -y tini openssl sqlite3
COPY --from=builder /app/target/release/oca-repository /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["tini", "--"]
CMD /usr/local/bin/oca-repository
