# Developer Guide

## Prerequisites

- Rust stable toolchain (edition 2021)
- Docker & Docker Compose (optional, for containerized development)
- Git

## Initial Setup

```bash
git clone git@github.com:THCLab/oca-repository-rs.git
cd oca-repository-rs

# Initialize and fetch the ocafile-examples submodule (required for e2e tests)
git submodule update --init --recursive

# Install the pre-commit hook
cp scripts/pre-commit .git/hooks/pre-commit
```

## Building & Running

```bash
# Debug build
cargo build

# Run locally (starts on port 8000, reads config/config.yml)
cargo run

# Release build
cargo build --release
```

The application reads configuration from `config/config.yml` relative to the current working directory. Always run from the project root.

## Testing

### Unit / integration tests

```bash
cargo test
```

### End-to-end tests

E2e tests (`tests/e2e_api.rs`) require the **ocafile-examples** submodule to be initialized. They:

1. Build the binary and spawn it on a random port with a temp directory
2. POST an example OCAFile from `ocafile-examples/2.0/specification/examples.ocafile`
3. Exercise all major API endpoints (bundle CRUD, search, steps, ocafile, objects, explore, data-entry)

If the submodule is not initialized, the e2e test will panic with `ocafile example not found`.

### Linting

```bash
# Clippy (CI enforces this with -D warnings — warnings are errors)
cargo clippy -- -D warnings

# Fast compile check
cargo check
```

## Docker

```bash
# Run with Docker Compose (oca-repository on :8000, Swagger UI on :8080)
docker compose up

# Build image manually
docker build -t oca-repository .
```

## Core Overlays Sync

The file `core_overlays/semantic.overlayfile` is a copy of the source of truth located in the **oca-rs** repository at `../oca-rs/overlay-file/core_overlays/semantic.overlayfile`.

### Keeping it in sync

The **pre-commit hook** (`scripts/pre-commit`) automatically checks whether the file is up to date before every commit. If it has drifted, the commit is blocked with a diff and instructions.

To sync manually:

```bash
./update_core_overlays.sh
```

This script:
- Compares `core_overlays/semantic.overlayfile` against the oca-rs source
- Copies the file if it differs
- Exits with an error if the oca-rs repository is not present alongside this repo

### Re-installing the hook

After a fresh clone:

```bash
cp scripts/pre-commit .git/hooks/pre-commit
```

## Project Structure

```
src/
├── main.rs              # Entry point: config, DB init, server start
├── lib.rs               # Module declarations
├── startup.rs           # Actix-web AppState, route definitions, server
├── configuration.rs     # Config structs + YAML loader
├── cache.rs             # OCAFile deduplication cache (SHA-256 → SAID)
├── logging.rs           # Tracing initialization (stderr or rotating file)
├── data_storage.rs      # Legacy/unused Sled DataStorage
├── ledger.rs            # Legacy/unused MicroLedger
└── routes/
    ├── mod.rs
    ├── health_check.rs  # GET /health_check
    ├── oca_bundles.rs   # Core OCA bundle API
    ├── objects.rs       # Batch object fetch
    ├── explore.rs       # Relation graph
    └── internal.rs      # Paginated list endpoints
tests/
└── e2e_api.rs           # End-to-end integration tests
scripts/
└── pre-commit           # Git pre-commit hook for overlay sync
config/
└── config.yml           # Application configuration
core_overlays/
└── semantic.overlayfile # Core overlay definitions (synced from oca-rs)
```

## Release Process

Releases are managed via `cargo release` (see `release.toml`). The workflow:

1. `cargo release` bumps version, updates `CHANGELOG.md` via `git-cliff`, commits, and tags
2. Tag push triggers `.github/workflows/build.yml` which builds and pushes a Docker image to `ghcr.io/thclab/oca-repository`
3. A GitHub Release is created with the changelog

## Repository Layout (Sibling Repos)

This repository expects to live alongside other OCA ecosystem repos under the same parent directory:

```
sources/oca/
├── oca-rs/                  # OCA Rust SDK (source of core overlay definitions)
├── oca-repository-rs/       # ← this repo
├── oca-store/               # Storage layer
├── oca-sdk-rs/              # High-level OCA SDK
└── ocafile-examples/        # (submodule) Example OCAFiles used in tests
```

The `update_core_overlays.sh` script and pre-commit hook use relative paths (`../oca-rs/...`) to locate the sibling repository.
