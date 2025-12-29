//! United Nations [Standard country or area codes for statistical use (M49)](https://unstats.un.org/unsd/methodology/m49/)
//!
//! This module contains parsers and data structures for the UN M49 standard, which is the primary
//! source for the ISO 3166-1 and 3166-2 standards. These standards do not, however, cost CHF300
//! (Swiss Francs) to download.

use serde::{Deserialize, Serialize, de::Deserializer};

/// A M49 CSV Record.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Record {
    /// The global numeric code
    #[serde(alias = "Global Code")]
    pub global_code: u16,

    /// The global name
    #[serde(alias = "Global Name")]
    pub global_name: String,

    /// The numeric region code
    #[serde(alias = "Region Code")]
    pub region_code: Option<u16>,

    /// The alphanumeric region name
    #[serde(alias = "Region Name")]
    pub region_name: Option<String>,

    /// The numeric sub-region code
    #[serde(alias = "Sub-region Code")]
    pub subregion_code: Option<u16>,

    /// The alphanumeric sub-region name
    #[serde(alias = "Sub-region Name")]
    pub subregion_name: Option<String>,

    /// The optional numeric intermediate region code
    #[serde(alias = "Intermediate Region Code")]
    pub intermediate_region_code: Option<u16>,

    /// The optional alphanumeric intermediate region code
    #[serde(alias = "Intermediate Region Name")]
    pub intermediate_region_name: Option<String>,

    /// The country name
    #[serde(alias = "Country or Area")]
    pub country_or_area: String,

    /// The numeric code for this country
    #[serde(alias = "M49 Code")]
    pub m49_code: u16,

    /// The ISO alpha-2 code
    #[serde(alias = "ISO-alpha2 Code")]
    pub alpha2: String,

    /// The ISO alpha-3 code
    #[serde(alias = "ISO-alpha3 Code")]
    pub alpha3: String,

    /// Whether the UN considers this country one of the least developed
    #[serde(
        alias = "Least Developed Countries (LDC)",
        deserialize_with = "deserialize_bool"
    )]
    pub least_developed_country: bool,

    /// Whether this country is landlocked, and the UN considers it developing
    #[serde(
        alias = "Land Locked Developing Countries (LLDC)",
        deserialize_with = "deserialize_bool"
    )]
    pub landlocked_developing_country: bool,

    /// Whether this country is defined as a small island, and the UN considers it developing
    #[serde(
        alias = "Small Island Developing States (SIDS)",
        deserialize_with = "deserialize_bool"
    )]
    pub small_island_developing_state: bool,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    Ok(!s.is_empty())
}

#[cfg(test)]
mod test {
    use super::Record;
    use csv::ReaderBuilder;

    /// The M49 dataset as of the second update from 2021, retrieved Sep. 5, 2025.
    const M49_2021_2: &str = include_str!("m49/2025-09-05.csv");

    #[test]
    fn count_2021_2() {
        const EXPECTED: usize = 248;

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .from_reader(M49_2021_2.as_bytes());
        let mut count = 0usize;

        for result in reader.deserialize::<Record>() {
            let record = result.expect("Could not parse record");
            eprintln!("{record:?}");
            count += 1;
        }

        assert_eq!(EXPECTED, count);
    }
}
