# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/jcape/iso3166/compare/iso3166-macros-v0.2.0...iso3166-macros-v0.3.0) - 2025-12-31

### Fixed

- rename syrianarabrepublic to syria

### Other

- bring back unit tests
- fix unused var warning
- [**breaking**] rewrite to support user-assigned codes

## [0.2.0](https://github.com/jcape/iso3166/compare/iso3166-macros-v0.1.1...iso3166-macros-v0.2.0) - 2025-12-27

### Added

- [**breaking**] make numeric enum serde used the numeric code
- add alpha2 and alpha3 types

### Other

- improve examples and static readme.
- fixup badges
- bump badges on release
- unit tests for numeric enum and serde
- move trait impls to static from macro

## [0.1.1](https://github.com/jcape/iso3166/compare/iso3166-macros-v0.1.0...iso3166-macros-v0.1.1) - 2025-12-21

### Added

- serde feature

### Fixed

- resolve clippy issues
