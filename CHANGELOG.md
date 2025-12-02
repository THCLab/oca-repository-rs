## [2.0.0-rc.1] - 2025-12-02

### 🚀 Features

- Update to oca 2.0.0

### 🐛 Bug Fixes

- SemVer

### 💼 Other

- Switch to oca sdk rs
- Fix support for overlayfile

### ⚙️ Miscellaneous Tasks

- Update dependencies
## [0.8.1] - 2025-03-25

### 🚀 Features

- Add limit query parameter to internal endpoints

### ⚙️ Miscellaneous Tasks

- Release 0.8.1 version
## [0.8.0] - 2025-03-06

### ⚙️ Miscellaneous Tasks

- [**breaking**] Update oca package to 0.7.1
- Release 0.8.0 version
## [0.7.1] - 2025-01-28

### ⚙️ Miscellaneous Tasks

- [**breaking**] Update oca package to v0.6.10
- Release 0.7.1 version
## [0.7.0] - 2024-11-15

### 🚀 Features

- Add version to OCA Bundle

### ⚙️ Miscellaneous Tasks

- [**breaking**] Update oca-rs package to 0.6.0
- Release 0.7.0 version
## [0.6.0] - 2024-10-11

### 🚀 Features

- Cache built ocafiles
- Allow setting cache path in config file

### 🚜 Refactor

- Fix clippy warnings
- Remove unused code

### ⚙️ Miscellaneous Tasks

- Release 0.6.0 version
## [0.5.10] - 2024-08-20

### 🐛 Bug Fixes

- Time package version

### ⚙️ Miscellaneous Tasks

- Update said package to 0.4.1 and oca-rs package to 0.5.4
- Fix release.toml
- Release 0.5.10 version
## [0.5.9] - 2024-07-22

### ⚙️ Miscellaneous Tasks

- Update README.md
- Auto bump README docker img version
- Release 0.5.9 version
## [0.5.8] - 2024-07-19

### 🐛 Bug Fixes

- Serialization of GET /oca-bundles response to include bundle version

### ⚙️ Miscellaneous Tasks

- Release 0.5.8 version
## [0.5.7] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Set no-print-matched-heading
- Release 0.5.7 version
## [0.5.6] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Use proper semver
- Release 0.5.6 version
## [0.5.5] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Set proper img link
- Release 0.5.5 version
## [0.5.4] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Display changelog
- Release 0.5.4 version
## [0.5.3] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Update build.yml
- Release 0.5.2 version
- Move extract-changelog before build
- Release 0.5.3 version
## [0.5.1] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Update build.yml
- Release 0.5.1 version
## [0.5.0] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Update readme and docker-compose
- Add release.toml
- Add changelog
- Release 0.5.0 version
## [0.5.0-rc.8] - 2024-07-19

### ⚙️ Miscellaneous Tasks

- Add dockerfile description
## [0.5.0-rc.6] - 2024-07-11

### ⚙️ Miscellaneous Tasks

- Update oca packages to 0.4.5
## [0.5.0-rc.5] - 2024-07-02

### ⚙️ Miscellaneous Tasks

- Update oca packages to 0.4.4
## [0.5.0-rc.4] - 2024-02-16

### 🐛 Bug Fixes

- Return UnprocessableEntity response with error message when parsing SelfAddressingIdentifier fails
## [0.5.0-rc.3] - 2024-01-11

### ⚙️ Miscellaneous Tasks

- Bump oca-rs to 0.4.1-rc.5
## [0.5.0-rc.2] - 2024-01-04

### ⚙️ Miscellaneous Tasks

- Update oca-praser-xls
## [0.5.0-rc.1] - 2024-01-03

### 🚀 Features

- Add support for bundle deps
- Bump to oca-rs 0.4.1-rc.1

### 💼 Other

- Bump to 0.5.0
## [0.4.19] - 2023-11-03

### 🐛 Bug Fixes

- Allow to access oca_facade even if is poisoned
## [0.4.18] - 2023-10-26

### 🐛 Bug Fixes

- Handle error cases when adding invalid ocafile

### ⚙️ Miscellaneous Tasks

- Revert "refactor: replace Arc with web::Data for oca_facade_web_data variable"
- Disable clippy warning
## [0.4.17] - 2023-10-03

### 🚜 Refactor

- Replace Arc with web::Data for oca_facade_web_data variable
## [0.4.16] - 2023-10-03

### 🚀 Features

- Use FileSystemStorage as db_cache in oca_rs::Facade
## [0.4.15] - 2023-09-28

### 🚀 Features

- Update /explore/{said} endpoint to return overlay metadata
- Add /objects endpoint
## [0.4.14] - 2023-09-18

### 🚀 Features

- Add /explore/{said} endpoint
## [0.4.13] - 2023-09-12

### 🚀 Features

- Add support for pagination in /internal/oca-bundles and /internal/capture-bases endpoints

### 🐛 Bug Fixes

- Add Sync trait to data_storage parameter to allow for concurrent access
## [0.4.12] - 2023-09-11

### 🚀 Features

- Add support for pagination in search endpoint
- Add support for language parameter in search endpoint
- Add new internal routes for fetching all capture bases and oca bundles

### 🐛 Bug Fixes

- Update code to use EncodeBundle trait and improve OCABundle JSON serialization
## [0.4.11] - 2023-08-30

### 🚀 Features

- Add support for optional feature 'data_entries_xls' in Cargo.toml and Dockerfile
## [0.4.10] - 2023-08-30

### 🚀 Features

- Update oca-rs to 0.3.0-rc.11 for search limit support
## [0.4.9] - 2023-08-30

### 🚀 Features

- Add support for downloading OCA Bundle data entry XLSX file
## [0.4.8] - 2023-08-28

### 🚀 Features

- Add /oca-bundles/search endpoint

### 🐛 Bug Fixes

- *(openapi.yml)* Move `extend` query param from /.../ocafile endpoint to /.../steps
## [0.4.7] - 2023-08-18

### ⚙️ Miscellaneous Tasks

- Rename `expand` query param to `extend`
## [0.4.6] - 2023-08-18

### 🚀 Features

- Add `expand` query parameter to /oca-bundles/{said}/steps endpint
## [0.4.5] - 2023-08-04

### ⚙️ Miscellaneous Tasks

- *(Cargo.toml)* Update oca-rs dependency to version 0.3.0-rc.8
## [0.4.4] - 2023-07-14

### 🚀 Features

- Add /oca-bundles/{said}/ocafile endpoint
## [0.4.3] - 2023-07-14

### 🚀 Features

- Use oca_rs::Facade in /oca_bundle endpoints

### ⚙️ Miscellaneous Tasks

- Rename /oca-bundle enpoints to /oca-bundles
## [0.4.2] - 2023-07-12

### 🚀 Features

- Use oca_bundle::build::from_ast in add_oca_file and handle errors
## [0.4.0] - 2023-07-11

### ⚙️ Miscellaneous Tasks

- Apply clippy suggestions
- Update docker image name
- Update said and oca-* versions
## [0.1.0] - 2023-07-03

### 🚀 Features

- Update add_oca_file function to use oca_dag for building versioned OCA Bundles
- Add /oca-bundle/{said}/steps endpoint to retrieve OCAFile history for a given OCA Bundle
- Add CI workflow for building and pushing docker images

### 📚 Documentation

- Add OpenAPI specification
- Hide /namespaces enpoints in openapi

### ⚙️ Miscellaneous Tasks

- Update path for config.yml
