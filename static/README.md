# ISO 3166 Static Data

This crate provides a generated enumeration for use as an ISO 3166 code enum

## Examples

```rust
use iso3166_static::Country;

let country1 = Country::try_from_alpha2("US").expect("alpha2");
let country2 = Country::try_from_alpha3("USA").expect("alpha3");

assert_eq!(country1, country2);
```

The [`FromStr`](::core::str::FromStr) implementation is more forgiving, as it will
remove any leading/trailing whitespace, and converting to an upper-case string.

```rust
use core::str::FromStr;
use iso3166_static::Country;

let country1 = Country::from_str("  US  ").expect("trimmed ascii");
let country2 = Country::from_str("usa").expect("case insenitive");

assert_eq!(country1, country2);
```

Some failure conditions:

```rust
use core::str::FromStr;
use iso3166_static::Country;

let _ = Country::try_from_alpha2("us").expect_err("not uppercase");
let _ = Country::try_from_alpha3("usa").expect_err("not uppercase");
let _ = Country::from_str("asdf").expect_err("not a code");
```
