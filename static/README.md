# ISO 3166 Static Data

[![Crates][crates-image]][crates-link]<!--
-->[![Docs][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->![License][license-image]

This crate provides a generated enumeration for use as an ISO 3166-1 code enum, and some wrapper types with more strict conventions. This crate is both no-std and no-alloc (with no need/desire to enable them), and supports serde via the `"serde"` feature.

There are three primary objects in this crate:

- [`Numeric`](crate::Numeric) - The ISO 3166-1 numeric country codes.
- [`Alpha2`](crate::Alpha2) - A newtype wrapper which strictly enforces the use of Alpha2 strings.
- [`Alpha3`](crate::Alpha3) - A newtype wrapper which strictly enforces the use of Alpha3 strings.

## Features

By default, this crate compiles with `serde` enabled, and `alloc` disabled. If your compilation enables the `alloc` feature on the `serde` crate, you should enable it here as well to prevent deserialization failures.

- `default`: Enables the `serde` feature by default.
- `alloc`: Enables the use of the `alloc` crate.
- `serde`: Enables implementations of the [`serde::Deserialize`] and [`serde::Serialize] traits.

## Examples

```rust
use iso3166_static::{Alpha2, Alpha3, Numeric};

const USA_ALPHA2: &str = "US";

let alpha2 = Alpha2::from_alpha2(USA_ALPHA2).expect("alpha2");
let alpha3 = Alpha3::from(alpha2.clone());
let numeric = Numeric::from(alpha3.clone());

assert_eq!(USA_ALPHA2, alpha2.as_str());
assert_eq!(alpha2, alpha3);
assert_eq!(alpha2, numeric);
assert_eq!(alpha3, numeric);
```

```rust
use core::str::FromStr;
use iso3166_static::{Alpha2, Alpha3, Numeric};

let USA_ALPHA3: &str = "USA";

let numeric = Numeric::from_str(USA_ALPHA3).expect("numeric");
let alpha3 = Alpha3::from(numeric.clone());
let alpha2 = Alpha2::from(numeric.clone());

assert_eq!(alpha3.as_str(), USA_ALPHA3);
assert_eq!(numeric, alpha3);
assert_eq!(numeric, alpha2);
assert_eq!(alpha3, alpha2);
```

[//]: # (badges)

[crates-image]: <https://img.shields.io/crates/v/iso3166-static?style=for-the-badge>
[crates-link]: <https://crates.io/crates/iso3166-static/0.2.0>
[docs-image]: <https://img.shields.io/docsrs/iso3166-static/0.2.0?style=for-the-badge>
[docs-link]: <https://docs.rs/iso3166-static/0.2.0/iso3166_static/>
[deps-image]: <https://img.shields.io/deps-rs/iso3166-static/0.2.0?style=for-the-badge>
[deps-link]: <https://deps.rs/crate/iso3166-static/0.2.0>
[license-image]: <https://img.shields.io/crates/l/iso3166-static?style=for-the-badge>
