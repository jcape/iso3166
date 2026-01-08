# ISO 3166 Static Data

[![Crates][crates-image]][crates-link]<!--
-->[![Docs][docs-image]][docs-link]<!--
-->[![Dependency Status][deps-image]][deps-link]<!--
-->![License][license-image]

This crate provides generated enumerations for use as with ISO 3166-1 codes. This crate is both `no-std` and `no-alloc` (with no need/desire to enable them), and supports serde via the `"serde"` feature.

There are three primary objects in this crate:

- [`Numeric`](crate::Numeric) - Numeric country codes.
- [`Alpha2`](crate::Alpha2) - Two-character country codes.
- [`Alpha3`](crate::Alpha3) - Three-character country codes.

## Features

By default, this crate compiles with `serde` enabled, and `alloc` disabled. If your compilation enables the `alloc` feature on the `serde` crate, you should enable it here as well to prevent deserialization failures.

- `default`: Enables the `serde` feature by default.
- `alloc`: Enables the use of the `alloc` crate.
- `serde`: Enables implementations of the [`serde::Deserialize`] and [`serde::Serialize`] traits.

## Examples

```rust
use iso3166_static::{Alpha2, Alpha3, Numeric};

const USA_ALPHA2: &str = "US";

let alpha2 = Alpha2::try_from(USA_ALPHA2).expect("alpha2");
let alpha3 = Alpha3::try_from(alpha2.clone()).expect("alpha3");
let numeric = Numeric::try_from(alpha3.clone()).expect("numeric");

assert_eq!(USA_ALPHA2, alpha2.as_str());
assert_eq!(alpha2, alpha3);
assert_eq!(alpha2, numeric);
assert_eq!(alpha3, numeric);
```

```rust
use core::str::FromStr;
use iso3166_static::{Alpha2, Alpha3, Numeric};

let USA_ALPHA3: &str = "USA";

let numeric = Numeric::UnitedStatesOfAmerica;
let alpha3 = Alpha3::try_from(numeric.clone()).expect("alpha3");
let alpha2 = Alpha2::try_from(numeric.clone()).expect("alpha2");

assert_eq!(alpha3.as_str(), USA_ALPHA3);
assert_eq!(numeric, alpha3);
assert_eq!(numeric, alpha2);
assert_eq!(alpha3, alpha2);
```

[//]: # (badges)

[crates-image]: <https://img.shields.io/crates/v/iso3166-static?style=flat-square>
[crates-link]: <https://crates.io/crates/iso3166-static/0.3.1>
[docs-image]: <https://img.shields.io/docsrs/iso3166-static/0.3.1?style=flat-square>
[docs-link]: <https://docs.rs/iso3166-static/0.3.1/iso3166_static/>
[deps-image]: <https://img.shields.io/deps-rs/iso3166-static/0.3.1?style=flat-square>
[deps-link]: <https://deps.rs/crate/iso3166-static/0.3.1>
[license-image]: <https://img.shields.io/crates/l/iso3166-static?style=flat-square>
