# OCA Repository

## Installation

1. Pull the Docker image for OCA Repository

```
docker pull ghcr.io/thclab/oca-repository:0.5.9
```

2. Run an instance of OCA Repository

```
docker run
    -it
    --rm
    -p 8000:8000
    -v $PWD/config/config.yml:/app/config/config.yml:ro
    humancolossus/oca-repository:0.5.9
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

Get OCA Bundle:

```
curl http://127.0.0.1:8000/oca-bundles/EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs
```
