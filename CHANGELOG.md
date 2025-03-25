# Changelog

## [0.8.1] - 2025-03-25

### Feat


- 
add limit query parameter to internal endpoints

(dcfa242)


## [v0.8.0] - 2025-03-06

### Chore


- 
update oca package to 0.7.1

(884c4f9)

- 
release 0.8.0 version

(6903dc1)


## [v0.7.1] - 2025-01-28

### Chore


- 
update oca package to v0.6.10

(b8391e3)

- 
release 0.7.1 version

(1307487)


## [v0.7.0] - 2024-11-15

### Chore


- 
update oca-rs package to 0.6.0

(e8b0e3e)

- 
release 0.7.0 version

(4c9031d)



### Feat


- 
add version to OCA Bundle

(706d61e)


## [v0.6.0] - 2024-10-11

### Chore


- 
release 0.6.0 version

(1ec02ed)



### Feat


- 
cache built ocafiles

(abf07b8)

- 
allow setting cache path in config file

(d451f36)



### Refactor


- 
fix clippy warnings

(83d4939)

- 
remove unused code

(6e7775d)


## [v0.5.10] - 2024-08-20

### Chore


- 
update said package to 0.4.1 and oca-rs package to 0.5.4

(89d8975)

- 
release 0.5.10 version

(a3ffe80)



### Ci


- 
fix release.toml

(6cf60a9)



### Fix


- 
time package version

(d687e78)


## [v0.5.9] - 2024-07-22

### Chore


- 
Update README.md

(ab7ab59)

- 
auto bump README docker img version

(0360bd4)

- 
release 0.5.9 version

(520bae7)


## [v0.5.8] - 2024-07-19

### Chore


- 
release 0.5.8 version

(5fba455)



### Fix


- 
serialization of GET /oca-bundles response to include bundle version

(15a5976)


## [v0.5.7] - 2024-07-19

### Chore


- 
set no-print-matched-heading

(9b71278)

- 
release 0.5.7 version

(94b16d1)


## [v0.5.6] - 2024-07-19

### Chore


- 
use proper semver

(c9f1ef8)

- 
release 0.5.6 version

(b48c107)


## [v0.5.5] - 2024-07-19

### Chore


- 
set proper img link

(6d59eef)

- 
release 0.5.5 version

(3f2d002)


## [v0.5.4] - 2024-07-19

### Chore


- 
display changelog

(0c02c78)

- 
release 0.5.4 version

(bebb94e)


## [v0.5.3] - 2024-07-19

### Chore


- 
Update build.yml

(fb90b19)

- 
release 0.5.2 version

(43b1b20)

- 
move extract-changelog before build

(e32f597)

- 
release 0.5.3 version

(4d3b158)


## [v0.5.1] - 2024-07-19

### Chore


- 
Update build.yml

(244e84d)

- 
release 0.5.1 version

(ca74e37)


## [v0.5.0] - 2024-07-19

### Chore


- 
update readme and docker-compose

(02fc535)

- 
add release.toml

(9c4db93)

- 
add changelog

(727d5be)

- 
release 0.5.0 version

(0bfec11)


## [v0.5.0-rc.8] - 2024-07-19

### Chore


- 
add dockerfile description

(2cee505)


## [v0.5.0-rc.6] - 2024-07-11

### Chore


- 
update oca packages to 0.4.5

(391d4f2)


## [v0.5.0-rc.5] - 2024-07-02

### Chore


- 
update oca packages to 0.4.4

(55964aa)


## [v0.5.0-rc.4] - 2024-02-16

### Fix


- 
return UnprocessableEntity response with error message when parsing SelfAddressingIdentifier fails

(04f9fe0)


## [v0.5.0-rc.3] - 2024-01-11

### Chore


- 
bump oca-rs to 0.4.1-rc.5

(becb5ab)


## [v0.5.0-rc.2] - 2024-01-04

### Chore


- 
update oca-praser-xls

(324b2bf)


## [v0.5.0-rc.1] - 2024-01-03

### Feat


- 
add support for bundle deps

(c60ae9b)

- 
bump to oca-rs 0.4.1-rc.1

(d09b8de)



### Release


- 
Bump to 0.5.0

(3c5bfab)


## [v0.4.19] - 2023-11-03

### Fix


- 
allow to access oca_facade even if is poisoned

(74e682a)


## [v0.4.18] - 2023-10-26

### Chore


- 
Revert "refactor: replace Arc with web::Data for oca_facade_web_data variable"

(08bbd77)

- 
disable clippy warning

(d664b27)



### Fix


- 
handle error cases when adding invalid ocafile

(7861044)


## [v0.4.17] - 2023-10-03

### Refactor


- 
replace Arc with web::Data for oca_facade_web_data variable

(2471335)


## [v0.4.16] - 2023-10-03

### Feat


- 
use FileSystemStorage as db_cache in oca_rs::Facade

(9767fd1)


## [v0.4.15] - 2023-09-28

### Feat


- 
update /explore/{said} endpoint to return overlay metadata

(9e01184)

- 
add /objects endpoint

(4806479)


## [v0.4.14] - 2023-09-18

### Feat


- 
add /explore/{said} endpoint

(83fecba)


## [v0.4.13] - 2023-09-12

### Feat


- 
add support for pagination in /internal/oca-bundles and /internal/capture-bases endpoints

(8b736e9)



### Fix


- 
add Sync trait to data_storage parameter to allow for concurrent access

(2b70723)


## [v0.4.12] - 2023-09-11

### Feat


- 
add support for pagination in search endpoint

(e0744af)

- 
add support for language parameter in search endpoint

(a2e85c2)

- 
add new internal routes for fetching all capture bases and oca bundles

(26ca197)



### Fix


- 
update code to use EncodeBundle trait and improve OCABundle JSON serialization

(e5e6c7b)


## [v0.4.11] - 2023-08-30

### Feat


- 
add support for optional feature 'data_entries_xls' in Cargo.toml and Dockerfile

(075ebcf)


## [v0.4.10] - 2023-08-30

### Feat


- 
update oca-rs to 0.3.0-rc.11 for search limit support

(b0a5135)


## [v0.4.9] - 2023-08-30

### Feat


- 
add support for downloading OCA Bundle data entry XLSX file

(531bc43)


## [v0.4.8] - 2023-08-28

### Feat


- 
add /oca-bundles/search endpoint

(bea140d)



### Fix


- 
move `extend` query param from /.../ocafile endpoint to /.../steps

(65c4bd5)


## [v0.4.7] - 2023-08-18

### Chore


- 
rename `expand` query param to `extend`

(beb21fc)


## [v0.4.6] - 2023-08-18

### Feat


- 
add `expand` query parameter to /oca-bundles/{said}/steps endpint

(15fa2fa)


## [v0.4.5] - 2023-08-04

### Chore


- 
update oca-rs dependency to version 0.3.0-rc.8

(268e60e)


## [v0.4.4] - 2023-07-14

### Feat


- 
add /oca-bundles/{said}/ocafile endpoint

(75c479e)


## [v0.4.3] - 2023-07-14

### Chore


- 
rename /oca-bundle enpoints to /oca-bundles

(c3c583a)



### Feat


- 
use oca_rs::Facade in /oca_bundle endpoints

(976ecd3)


## [v0.4.2] - 2023-07-12

### Feat


- 
use oca_bundle::build::from_ast in add_oca_file and handle errors

(70645ab)


## [v0.4.1] - 2023-07-11
## [v0.4.0] - 2023-07-11

### Chore


- 
apply clippy suggestions

(50fd1f2)

- 
update said and oca-* versions

(dca18fd)



### Ci


- 
update docker image name

(219753f)


## [v0.1.0] - 2023-07-03

### Chore


- 
update path for config.yml

(2c1d421)



### Docs


- 
add OpenAPI specification

(b81e5bc)

- 
hide /namespaces enpoints in openapi

(0d30535)



### Feat


- 
update add_oca_file function to use oca_dag for building versioned OCA Bundles

(fd86642)

- 
add /oca-bundle/{said}/steps endpoint to retrieve OCAFile history for a given OCA Bundle

(9824418)

- 
add CI workflow for building and pushing docker images

(f69aa4a)



