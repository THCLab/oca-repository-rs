# OCA Repository

## Installation

1. Pull the Docker image for OCA Repository (be aware to NOT use `latest` tag for production deployment)

```
docker pull ghcr.io/thclab/oca-repository:0.8.0
```
2. Customize config file

config.yml:

```
application:
  host: 0.0.0.0
  port: 8000
  data_entries_path: "/tmp"

database:
  path: "./db"

cache_storage:
  path: "./cache"

search_engine:
  path: "./db_search"

ocafiles_cache:
  path: "./oca_repo_cache"
```

2. Run an instance of OCA Repository

```
docker run -it --rm -p 8000:8000 -v $PWD/config.yml:/app/config/config.yml:ro ghcr.io/thclab/oca-repository:0.8.0
```

## Usage

See https://oca.colossi.network/ecosystem/oca-repository.html

## Development

With Docker and latest release

```
git clone git@github.com:THCLab/oca-repository-rs.git

docker-compose up
```

Locally with local build:

```
cargo build
```

```
./target/debug/oca-repository
```

This would start on default port `8000` insance of repository.
You can then use `curl` for playing with api or simply go to `https://repository.oca.argo.colossi.network/`
and switch servers to `localhost:8000` and use swagger.

Add OCA Bundle:

```
curl -XPOST http://127.0.0.1:8000/oca-bundles \
  -H "Content-type: text/plain" \
  --data-binary @ocafile.txt
```

If you need examples of OCA for development and testing check: https://github.com/thclab/ocafile-examples

Get OCA Bundle:

```
curl http://127.0.0.1:8000/oca-bundles/EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs
```
