# OCA Repository

## Usage

...

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
