# OCA Repository

## Usage

1. Pull the Docker image for OCA Repository
```
docker pull humancolossus/oca-repository-rs:latest
```
Change `latest` to a [ SemVer ](https://semver.org/)-based tag for production deployment.
2. Run an instance of OCA Repository
```
docker run
    -p 8000:8000
    -v $PWD/config/config.yml:/app/config/config.yml:ro
    humancolossus/oca-repository-rs:latest
```
3. See the `openapi.yml` spec under the `OCA Repository` tag for all available endpoints.


## Core components

...

## Development

```
git clone git@github.com:THCLab/oca-repository-rs.git

docker-compose up
```

Add OCA Bundle:
```
curl -XPOST http://127.0.0.1:8000/oca-bundle \
  -H "Content-type: text/plain" \
  --data-binary @ocafile.txt
```

Get OCA Bundle:
```
curl http://127.0.0.1:8000/oca-bundle/EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs
```
