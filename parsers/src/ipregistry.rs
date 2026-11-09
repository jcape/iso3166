//! Parsing support for the CSV files published in the
//! [ipregistry/iso3166](https://github.com/ipregistry/iso3166) repository.
//!
//! Each file has a header row whose first column name starts with a `#`. Because the files are
//! plain CSV, that header is still consumed as the header row by [`csv`](https://docs.rs/csv),
//! so the `#` is part of the first field's name; the records here rename that field accordingly.

use serde::{Deserialize, Serialize};

/// A CSV record in the `administrative-languages.csv` file.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AdministrativeLanguage {
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "#country_code")]
    pub country_code: String,
    /// The ISO 639-1 (two letter) language code.
    pub alpha2_code: String,
    /// The ISO 639-2 (three letter) language code.
    pub alpha3_code: String,
    /// The short name of the country, in the administrative language.
    pub short_name: String,
}

/// A CSV record in the `countries-sovereignty.csv` file.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CountrySovereignty {
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "#country_code")]
    pub country_code: String,
    /// Whether the country is independent.
    pub is_independent: bool,
    /// The ISO 3166-1 alpha-2 code of the sovereign country, if this country is not independent.
    pub sovereign_country_code: Option<String>,
}

/// A CSV record in the `countries.csv` file.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Country {
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "#country_code_alpha2")]
    pub alpha2: String,
    /// The ISO 3166-1 alpha-3 country code.
    #[serde(rename = "country_code_alpha3")]
    pub alpha3: String,
    /// The ISO 3166-1 numeric country code.
    pub numeric_code: u16,
    /// The short, common name of the country.
    pub name_short: String,
    /// The long, official name of the country.
    pub name_long: String,
}

/// A CSV record in the `subdivisions.csv` file.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Subdivision {
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(rename = "#country_code_alpha2")]
    pub country_code: String,
    /// The full ISO 3166-2 subdivision code, e.g. `US-NY`.
    #[serde(rename = "subdivision_code_iso3166-2")]
    pub code: String,
    /// The human-readable subdivision name.
    #[serde(rename = "subdivision_name")]
    pub name: String,
    /// The language code of the subdivision name.
    pub language_code: String,
    /// The ISO 3166-2 code of the parent subdivision, if any.
    #[serde(rename = "parent_subdivision")]
    pub parent: Option<String>,
    /// The category of the subdivision, e.g. `STATE` or `PROVINCE`.
    pub category: String,
    /// A local variant of the subdivision name, if any.
    #[serde(rename = "localVariant")]
    pub local_variant: Option<String>,
}

#[cfg(test)]
mod test {
    use super::{AdministrativeLanguage, Country, CountrySovereignty, Subdivision};
    use csv::ReaderBuilder;
    use serde::de::DeserializeOwned;

    const ADMINISTRATIVE_LANGUAGES: &str = include_str!("ipregistry/administrative-languages.csv");
    const COUNTRIES_SOVEREIGNTY: &str = include_str!("ipregistry/countries-sovereignty.csv");
    const COUNTRIES: &str = include_str!("ipregistry/countries.csv");
    const SUBDIVISIONS: &str = include_str!("ipregistry/subdivisions.csv");

    /// Deserialize every record of the given CSV data, returning them.
    fn parse<T: DeserializeOwned>(data: &str) -> Vec<T> {
        ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .from_reader(data.as_bytes())
            .deserialize::<T>()
            .collect::<Result<Vec<_>, _>>()
            .expect("Could not parse records")
    }

    #[test]
    fn administrative_languages_count() {
        const EXPECTED: usize = 347;

        assert_eq!(
            EXPECTED,
            parse::<AdministrativeLanguage>(ADMINISTRATIVE_LANGUAGES).len()
        );
    }

    #[test]
    fn countries_sovereignty_count() {
        const EXPECTED: usize = 249;

        assert_eq!(
            EXPECTED,
            parse::<CountrySovereignty>(COUNTRIES_SOVEREIGNTY).len()
        );
    }

    #[test]
    fn countries_count() {
        const EXPECTED: usize = 249;

        assert_eq!(EXPECTED, parse::<Country>(COUNTRIES).len());
    }

    #[test]
    fn subdivisions_count() {
        const EXPECTED: usize = 6260;

        assert_eq!(EXPECTED, parse::<Subdivision>(SUBDIVISIONS).len());
    }
}
