//! Parsing support for the IP-2-Location ISO3166-2 CSV.

use serde::{Deserialize, Serialize};

/// A CSV record in the IP2LOCATION-ISO3166-2.CSV file.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Record {
    /// The ISO3166-1 Alpha2 country code this subdivision represents.
    pub country_code: String,
    /// The Human-readable subdivision name.
    pub subdivision_name: String,
    /// The full ISO3166-2 country code, e.g. `US-NY`.
    ///
    /// If the trimmed contents of value are a single hyphen (`-`), that means there are no
    /// subdivisions for this country.
    pub code: String,
}

#[cfg(test)]
mod test {
    use super::Record;
    use csv::ReaderBuilder;

    const LOCATIONS: &str = include_str!("ip2location/IP2LOCATION-ISO3166-2.CSV");

    #[test]
    fn count() {
        const EXPECTED: usize = 3635;

        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .from_reader(LOCATIONS.as_bytes());
        let mut count = 0usize;

        for result in reader.deserialize::<Record>() {
            let record = result.expect("Could not parse record");
            eprintln!("{record:?}");
            count += 1;
        }

        assert_eq!(EXPECTED, count);
    }
}
