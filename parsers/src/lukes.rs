//! Serde support for Luke Duncalfe's combined JSON

use serde::{Deserialize, Serialize};

/// A record in the `all.json`.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Record {
    /// The name of the country record
    pub name: Option<String>,

    /// The Alpha2 Code
    #[serde(alias = "alpha-2")]
    pub alpha_2: Option<String>,

    /// The Alpha3 code
    #[serde(alias = "alpha-3")]
    pub alpha_3: Option<String>,

    /// The numeric code
    #[serde(alias = "country-code")]
    pub country_code: Option<String>,

    /// The ISO-3166-2 code
    #[serde(alias = "iso_3166-2")]
    pub iso_3166_2: Option<String>,

    /// The M49 region name
    pub region: Option<String>,
    #[serde(alias = "sub-region")]

    /// The M49 sub-region name
    pub sub_region: Option<String>,

    /// The M49 intermediate-region name
    #[serde(alias = "intermediate-region")]
    pub intermediate_region: Option<String>,

    /// The M49 region numeric code.
    #[serde(alias = "region-code")]
    pub region_code: Option<String>,

    /// The M49 sub-region numeric code.
    #[serde(alias = "sub-region-code")]
    pub sub_region_code: Option<String>,

    /// The M49 intermediate-region numeric code.
    #[serde(alias = "intermediate-region-code")]
    pub intermediate_region_code: Option<String>,
}

#[cfg(test)]
mod test {
    use super::Record;

    const ALL_JSON: &str = include_str!("lukes/2025-12-29.json");

    #[test]
    fn all_json() {
        let records = serde_json::from_str::<Vec<Record>>(ALL_JSON).expect("valid json");

        assert_eq!(249, records.len());
    }
}
