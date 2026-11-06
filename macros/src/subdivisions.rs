//! ISO 3166-2 subdivision records suitable for use in macros.

use heck::ToPascalCase as _;
use iso3166_parsers::ip2location::Record as Ip2LocationRecord;
use proc_macro2::Span;
use std::collections::BTreeMap;
use syn::{Error, Ident, Result};

/// A struct of arrays containing the subdivisions of a single country.
#[derive(Clone)]
pub(crate) struct Subdivisions {
    /// The ISO 3166-1 alpha-2 code of the country these subdivisions belong to.
    country_code: String,
    /// The enum variant identifier for each subdivision.
    ident: Vec<Ident>,
    /// The documentation for each subdivision, in the form `code - name`.
    doc: Vec<String>,
}

impl Subdivisions {
    /// Build a map of country codes to the subdivisions of that country from the given
    /// ip2location records.
    ///
    /// Records without a subdivision code, or with a code of `-`, are silently skipped.
    ///
    /// # Errors
    ///
    /// - When a subdivision name cannot be converted into a valid ASCII identifier.
    pub(crate) fn from_ip2location(
        span: Span,
        records: &[Ip2LocationRecord],
    ) -> Result<BTreeMap<String, Self>> {
        let mut retval = BTreeMap::<String, Self>::new();

        for record in records {
            let code = record.code.trim();
            if code.is_empty() || code == "-" {
                continue;
            }

            let country_code = record.country_code.trim();
            let name = record.subdivision_name.trim();
            let ident = name_to_ident(span, name)?;
            let doc = format!(" {code} - {name}");

            let entry = retval
                .entry(country_code.to_owned())
                .or_insert_with(|| Self {
                    country_code: country_code.to_owned(),
                    ident: Vec::new(),
                    doc: Vec::new(),
                });
            entry.ident.push(ident);
            entry.doc.push(doc);
        }

        Ok(retval)
    }

    /// Retrieve the ISO 3166-1 alpha-2 country code.
    pub(crate) fn country_code(&self) -> &str {
        &self.country_code
    }

    /// Retrieve the subdivision identifiers, as a slice.
    pub(crate) fn ident(&self) -> &[Ident] {
        &self.ident
    }

    /// Retrieve the documentation strings, as a slice.
    pub(crate) fn doc(&self) -> &[String] {
        &self.doc
    }
}

/// A helper function to convert a subdivision name into an identifier suitable for enum use.
#[expect(clippy::single_call_fn, reason = "Clean code.")]
fn name_to_ident(span: Span, name: &str) -> Result<Ident> {
    let ascii = name.trim().chars().fold(String::new(), |mut acc, ch| {
        match to_ascii(ch) {
            Some(replacement) => acc.push_str(replacement),
            None => acc.push(ch),
        }
        acc
    });
    let ident = ascii.to_pascal_case();

    if !ident.is_ascii() {
        let message = format!("Subdivision identifier {ident:?} from {name:?} is not ASCII");
        return Err(Error::new(span, message));
    }

    syn::parse_str::<Ident>(&ident).map_err(|error| {
        let message = format!("Invalid subdivision identifier {ident:?} from {name:?}: {error}");
        Error::new(span, message)
    })
}

/// Translate a non-ASCII character into an ASCII equivalent, if one is known.
#[expect(clippy::single_call_fn, reason = "Clean code.")]
fn to_ascii(ch: char) -> Option<&'static str> {
    let retval = match ch {
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' | 'Ă' | 'Ą' => "A",
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => "a",
        'Æ' => "AE",
        'æ' => "ae",
        'Ç' | 'Ć' | 'Ĉ' | 'Ċ' | 'Č' => "C",
        'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => "c",
        'Ď' | 'Đ' | 'Ð' => "D",
        'ď' | 'đ' | 'ð' => "d",
        'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ĕ' | 'Ė' | 'Ę' | 'Ě' => "E",
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => "e",
        'Ĝ' | 'Ğ' | 'Ġ' | 'Ģ' => "G",
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => "g",
        'Ĥ' | 'Ħ' => "H",
        'ĥ' | 'ħ' => "h",
        'Ì' | 'Í' | 'Î' | 'Ï' | 'Ĩ' | 'Ī' | 'Ĭ' | 'Į' | 'İ' => "I",
        'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => "i",
        'Ĵ' => "J",
        'ĵ' => "j",
        'Ķ' => "K",
        'ķ' => "k",
        'Ĺ' | 'Ļ' | 'Ľ' | 'Ŀ' | 'Ł' => "L",
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => "l",
        'Ñ' | 'Ń' | 'Ņ' | 'Ň' => "N",
        'ñ' | 'ń' | 'ņ' | 'ň' => "n",
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' | 'Ŏ' | 'Ő' => "O",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => "o",
        'Œ' => "OE",
        'œ' => "oe",
        'Ŕ' | 'Ŗ' | 'Ř' => "R",
        'ŕ' | 'ŗ' | 'ř' => "r",
        'Ś' | 'Ŝ' | 'Ş' | 'Š' | 'Ș' => "S",
        'ś' | 'ŝ' | 'ş' | 'š' | 'ș' => "s",
        'ß' => "ss",
        'Ţ' | 'Ť' | 'Ŧ' | 'Ț' => "T",
        'ţ' | 'ť' | 'ŧ' | 'ț' => "t",
        'Þ' => "Th",
        'þ' => "th",
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ũ' | 'Ū' | 'Ŭ' | 'Ů' | 'Ű' | 'Ų' => "U",
        'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => "u",
        'Ŵ' => "W",
        'ŵ' => "w",
        'Ý' | 'Ÿ' | 'Ŷ' => "Y",
        'ý' | 'ÿ' | 'ŷ' => "y",
        'Ź' | 'Ż' | 'Ž' => "Z",
        'ź' | 'ż' | 'ž' => "z",
        '‘' | '’' | 'ʻ' | 'ʼ' => "'",
        '‐' | '‑' | '–' | '—' => "-",
        _ => return None,
    };

    Some(retval)
}

#[cfg(test)]
mod test {
    use super::Subdivisions;
    use iso3166_parsers::ip2location::Record;
    use proc_macro2::Span;

    /// Build a record from the given fields.
    fn record(country_code: &str, subdivision_name: &str, code: &str) -> Record {
        Record {
            country_code: country_code.to_owned(),
            subdivision_name: subdivision_name.to_owned(),
            code: code.to_owned(),
        }
    }

    #[test]
    fn groups_and_filters() {
        let records = [
            record("US", "New York", "US-NY"),
            record("AQ", "Antarctica", "-"),
            record("CA", "Ontario", "CA-ON"),
            record("CA", "Québec", "CA-QC"),
            record("ZZ", "Nowhere", " "),
            record("US", "North Carolina", "US-NC"),
        ];

        let map = Subdivisions::from_ip2location(Span::call_site(), &records)
            .expect("Could not build subdivisions");

        assert_eq!(map.len(), 2);

        let us = map.get("US").expect("Missing US");
        assert_eq!(us.country_code(), "US");
        assert_eq!(us.doc(), [" US-NY - New York", " US-NC - North Carolina"]);
        assert_eq!(us.ident(), ["NewYork", "NorthCarolina"]);

        let ca = map.get("CA").expect("Missing CA");
        assert_eq!(ca.ident(), ["Ontario", "Quebec"]);
        assert_eq!(ca.doc(), [" CA-ON - Ontario", " CA-QC - Québec"]);
    }

    #[test]
    fn transliterates_names() {
        let records = [
            record("AX", "Ålands skärgård", "AX-01"),
            record("DE", "Thüringen", "DE-TH"),
            record("RO", "Timiș", "RO-TM"),
        ];

        let map = Subdivisions::from_ip2location(Span::call_site(), &records)
            .expect("Could not build subdivisions");

        assert_eq!(
            map.get("AX").expect("Missing AX").ident(),
            ["AlandsSkargard"]
        );
        assert_eq!(map.get("DE").expect("Missing DE").ident(), ["Thuringen"]);
        assert_eq!(map.get("RO").expect("Missing RO").ident(), ["Timis"]);
    }

    #[test]
    fn rejects_unknown_non_ascii() {
        let records = [record("JP", "東京都", "JP-13")];

        assert!(Subdivisions::from_ip2location(Span::call_site(), &records).is_err());
    }

    #[test]
    fn ip2location_csv() {
        const CSV: &str = include_str!("../../parsers/src/ip2location/IP2LOCATION-ISO3166-2.CSV");

        let records = csv::Reader::from_reader(CSV.as_bytes())
            .deserialize::<Record>()
            .collect::<Result<Vec<_>, _>>()
            .expect("Could not parse CSV");

        let map = Subdivisions::from_ip2location(Span::call_site(), &records)
            .expect("Could not build subdivisions");

        assert!(!map.contains_key("AQ"));
        assert_eq!(map.get("US").expect("Missing US").ident().len(), 51);
    }
}
