//! County records suitable for use in macros.

use heck::ToPascalCase as _;
use iso3166_parsers::lukes::Record as LukesRecord;
use proc_macro2::Span;
use syn::{Error, Ident, LitByteStr, LitStr, Result};

/// A struct of arrays of data used to expand the macro.
#[derive(Default, Clone)]
pub(crate) struct Countries {
    /// The enum variant identifier.
    ident: Vec<Ident>,
    /// The documentation for the given variant.
    doc: Vec<String>,
    /// The compact (u8) code.
    compact: Vec<u8>,
    /// The ISO standard numeric code.
    numeric: Vec<u16>,
    /// The alpha-2 code as a string literal.
    alpha2_str: Vec<LitStr>,
    /// The alpha-2 code, bytes.
    alpha2_bytes: Vec<LitByteStr>,
    /// The alpha-3 code, as a string literal.
    alpha3_str: Vec<LitStr>,
    /// The alpha-3 code, in bytes.
    alpha3_bytes: Vec<LitByteStr>,
}

impl Countries {
    /// Create an instance suitable for use in building user-specified numeric codes.
    #[expect(
        clippy::single_call_fn,
        reason = "Specialized constructor keeping country logic together"
    )]
    pub(crate) fn user_numeric() -> Self {
        const NUMERIC_USER_COUNT: usize = 100;

        let mut doc = Vec::with_capacity(NUMERIC_USER_COUNT);
        let mut ident = Vec::with_capacity(NUMERIC_USER_COUNT);
        let numeric = (900u16..=999).collect();

        for code in 900i16..=999 {
            let d = format!(" User-assigned {code}");
            let id = quote::format_ident!("User{code}");

            doc.push(d);
            ident.push(id);
        }

        Self {
            ident,
            doc,
            compact: Vec::default(),
            numeric,
            alpha2_str: Vec::default(),
            alpha2_bytes: Vec::default(),
            alpha3_str: Vec::default(),
            alpha3_bytes: Vec::default(),
        }
    }

    /// Create an instance suitable for use in building user-specified Alpha2 codes.
    #[expect(
        clippy::single_call_fn,
        reason = "Specialized constructor keeping country logic together"
    )]
    pub(crate) fn user_alpha2() -> Self {
        const ALPHA2_USER_COUNT: usize = 1 + 13 + 26;

        let mut doc = Vec::with_capacity(ALPHA2_USER_COUNT);
        let mut ident = Vec::with_capacity(ALPHA2_USER_COUNT);
        let mut alpha2_str = Vec::with_capacity(ALPHA2_USER_COUNT);
        let mut alpha2_bytes = Vec::with_capacity(ALPHA2_USER_COUNT);

        for first in ['A', 'Q', 'X', 'Z'] {
            if first == 'A' {
                // AA
                push_user_alpha2(
                    first,
                    'A',
                    &mut doc,
                    &mut ident,
                    &mut alpha2_str,
                    &mut alpha2_bytes,
                );
                continue;
            } else if first == 'Z' {
                // ZZ
                push_user_alpha2(
                    first,
                    'Z',
                    &mut doc,
                    &mut ident,
                    &mut alpha2_str,
                    &mut alpha2_bytes,
                );
                continue;
            } else if first == 'X' {
                // XA-XL
                for second in 'A'..='L' {
                    push_user_alpha2(
                        first,
                        second,
                        &mut doc,
                        &mut ident,
                        &mut alpha2_str,
                        &mut alpha2_bytes,
                    );
                }
            }

            // QM-QZ, XM-XZ
            for second in 'M'..='Z' {
                push_user_alpha2(
                    first,
                    second,
                    &mut doc,
                    &mut ident,
                    &mut alpha2_str,
                    &mut alpha2_bytes,
                );
            }
        }

        Self {
            ident,
            doc,
            compact: Vec::default(),
            numeric: Vec::default(),
            alpha2_str,
            alpha2_bytes,
            alpha3_str: Vec::default(),
            alpha3_bytes: Vec::default(),
        }
    }

    /// Create an instance suitable for use in building user-specified Alpha3 codes.
    ///
    /// Note: this will contain empty arrays for Compact, Numeric, and Alpha2 representations.
    #[expect(
        clippy::single_call_fn,
        reason = "Specialized constructor keeping country logic together"
    )]
    pub(crate) fn user_alpha3() -> Self {
        const ALPHA3_USER_COUNT: usize = 26 + (13 * 26) + (26 * 26) + (26);

        let mut doc = Vec::with_capacity(ALPHA3_USER_COUNT);
        let mut ident = Vec::with_capacity(ALPHA3_USER_COUNT);
        let mut alpha3_str = Vec::with_capacity(ALPHA3_USER_COUNT);
        let mut alpha3_bytes = Vec::with_capacity(ALPHA3_USER_COUNT);

        // AAA-AAZ (26)
        for pos3 in 'A'..='Z' {
            push_user_alpha3(
                'A',
                'A',
                pos3,
                &mut doc,
                &mut ident,
                &mut alpha3_str,
                &mut alpha3_bytes,
            );
        }

        // QMA-QZZ (13 * 26 = 338)
        for pos2 in 'M'..='Z' {
            for pos3 in 'A'..='Z' {
                push_user_alpha3(
                    'Q',
                    pos2,
                    pos3,
                    &mut doc,
                    &mut ident,
                    &mut alpha3_str,
                    &mut alpha3_bytes,
                );
            }
        }

        // XAA - XZZ (26 * 26 = 676)
        for pos2 in 'A'..='Z' {
            for pos3 in 'A'..='Z' {
                push_user_alpha3(
                    'X',
                    pos2,
                    pos3,
                    &mut doc,
                    &mut ident,
                    &mut alpha3_str,
                    &mut alpha3_bytes,
                );
            }
        }

        // ZZA-ZZZ (26)
        for code in 'A'..='Z' {
            push_user_alpha3(
                'Z',
                'Z',
                code,
                &mut doc,
                &mut ident,
                &mut alpha3_str,
                &mut alpha3_bytes,
            );
        }

        Self {
            ident,
            doc,
            compact: Vec::default(),
            numeric: Vec::default(),
            alpha2_str: Vec::default(),
            alpha2_bytes: Vec::default(),
            alpha3_str,
            alpha3_bytes,
        }
    }

    /// Create a new struct-of-arrays collection from the given lukes records.
    #[expect(
        clippy::single_call_fn,
        reason = "Specialized constructor keeping country logic together"
    )]
    pub(crate) fn from_lukes(records: &[LukesRecord], span: Span) -> Result<Self> {
        let mut retval = Self {
            ident: Vec::with_capacity(records.len()),
            doc: Vec::with_capacity(records.len()),
            compact: Vec::with_capacity(records.len()),
            numeric: Vec::with_capacity(records.len()),
            alpha2_str: Vec::with_capacity(records.len()),
            alpha2_bytes: Vec::with_capacity(records.len()),
            alpha3_str: Vec::with_capacity(records.len()),
            alpha3_bytes: Vec::with_capacity(records.len()),
        };

        for record in records {
            let name = record
                .name
                .as_deref()
                .ok_or_else(|| Error::new(span, "Missing country name in record"))?;
            let numeric = record
                .country_code
                .as_deref()
                .ok_or_else(|| Error::new(span, "Missing Country code in record"))?
                .parse::<u16>()
                .map_err(|err| Error::new(span, err.to_string()))?;
            let alpha2 = record
                .alpha_2
                .as_deref()
                .ok_or_else(|| Error::new(span, "Missing alpha-2 code in record"))?;
            let alpha3 = record
                .alpha_3
                .as_deref()
                .ok_or_else(|| Error::new(span, "Missing alpha-3 code in record"))?;

            let ident = name_to_ident(name);
            let doc = format!(" {name} ({alpha2}, {alpha3})");

            retval.ident.push(ident);
            retval.doc.push(doc);
            retval.compact.push(
                u8::try_from(retval.compact.len())
                    .map_err(|_error| Error::new(span, "More than 255 countries!"))?,
            );
            retval.numeric.push(numeric);
            retval
                .alpha2_str
                .push(LitStr::new(alpha2, Span::mixed_site()));
            retval
                .alpha2_bytes
                .push(LitByteStr::new(alpha2.as_bytes(), Span::mixed_site()));
            retval
                .alpha3_str
                .push(LitStr::new(alpha3, Span::mixed_site()));
            retval
                .alpha3_bytes
                .push(LitByteStr::new(alpha3.as_bytes(), Span::mixed_site()));
        }

        Ok(retval)
    }

    /// Retrieve the documentation strings, as a slice.
    pub(crate) fn doc(&self) -> &[String] {
        &self.doc
    }

    /// Retrieve the identifiers, as a slice.
    pub(crate) fn ident(&self) -> &[Ident] {
        &self.ident
    }

    /// Retrieve the numeric codes, as a slice.
    pub(crate) fn numeric(&self) -> &[u16] {
        &self.numeric
    }

    /// Retrieve the alpha2 byte string literals, as a slice.
    pub(crate) fn alpha2_str(&self) -> &[LitStr] {
        &self.alpha2_str
    }

    /// Retrieve the alpha2 byte string literals, as a slice.
    pub(crate) fn alpha2_bytes(&self) -> &[LitByteStr] {
        &self.alpha2_bytes
    }

    /// Retrieve the alpha3 string literals, as a slice.
    pub(crate) fn alpha3_str(&self) -> &[LitStr] {
        &self.alpha3_str
    }

    /// Retrieve the alpha3 byte string literals, as a slice.
    pub(crate) fn alpha3_bytes(&self) -> &[LitByteStr] {
        &self.alpha3_bytes
    }
}

/// Format the given code and append it to the given collections.
fn push_user_common(
    code: &str,
    doc: &mut Vec<String>,
    ident: &mut Vec<Ident>,
    string: &mut Vec<LitStr>,
    bytes: &mut Vec<LitByteStr>,
) {
    let d = format!(" User-assigned {code}");
    let id = quote::format_ident!("User{code}");
    let b = LitByteStr::new(code.as_bytes(), Span::mixed_site());
    let s = LitStr::new(code, Span::mixed_site());

    doc.push(d);
    ident.push(id);
    string.push(s);
    bytes.push(b);
}

/// A helper function to generate and append user-specified alpha2 values.
fn push_user_alpha2(
    first: char,
    second: char,
    doc: &mut Vec<String>,
    ident: &mut Vec<Ident>,
    alpha2_str: &mut Vec<LitStr>,
    alpha2_bytes: &mut Vec<LitByteStr>,
) {
    let mut code = String::new();
    code.push(first);
    code.push(second);

    push_user_common(&code, doc, ident, alpha2_str, alpha2_bytes);
}

/// A helper function to generate and append user-specified alpha3 codes.
fn push_user_alpha3(
    pos1: char,
    pos2: char,
    pos3: char,
    doc: &mut Vec<String>,
    ident: &mut Vec<Ident>,
    alpha3_str: &mut Vec<LitStr>,
    alpha3_bytes: &mut Vec<LitByteStr>,
) {
    let mut code = String::new();
    code.push(pos1);
    code.push(pos2);
    code.push(pos3);

    push_user_common(&code, doc, ident, alpha3_str, alpha3_bytes);
}

/// A helper function to convert a country name into an identifier suitable for enum use.
#[expect(clippy::single_call_fn, reason = "Clean code.")]
fn name_to_ident(name: &str) -> Ident {
    let ident = name
        .trim()
        .to_pascal_case()
        .replace("BoliviaPlurinationalStateOf", "Bolivia")
        .replace("VirginIslandsBritish", "BritishVirginIslands")
        .replace("TaiwanProvinceOfChina", "Taiwan")
        .replace(
            "CongoDemocraticRepublicOfThe",
            "DemocraticRepublicOfTheCongo",
        )
        .replace("ÅlandIslands", "AlandIslands")
        .replace("PalestineStateOf", "Palestine")
        .replace("IranIslamicRepublicOf", "Iran")
        .replace("CôteDIvoire", "CoteDIvoire")
        .replace("KoreaDemocraticPeopleSRepublicOf", "NorthKorea")
        .replace("KoreaRepublicOf", "SouthKorea")
        .replace("LaoPeopleSDemocraticRepublic", "Laos")
        .replace("MoldovaRepublicOf", "Moldova")
        .replace("NetherlandsKingdomOfThe", "Netherlands")
        .replace("Curaçao", "Curacao")
        .replace("MicronesiaFederatedStatesOf", "Micronesia")
        .replace("Réunion", "Reunion")
        .replace("RussianFederation", "Russia")
        .replace("SaintBarthélemy", "SaintBarthelemy")
        .replace("Türkiye", "Turkey")
        .replace(
            "UnitedKingdomOfGreatBritainAndNorthernIreland",
            "UnitedKingdom",
        )
        .replace("TanzaniaUnitedRepublicOf", "Tanzania")
        .replace("VenezuelaBolivarianRepublicOf", "Venezuela")
        .replace("SyrianArabRepublic", "Syria");

    quote::format_ident!("{ident}")
}
