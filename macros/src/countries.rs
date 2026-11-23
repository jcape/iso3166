//! County records suitable for use in macros.

use heck::ToPascalCase as _;
use iso3166_parsers::ipregistry::{Country as IpregistryCountry, CountrySovereignty};
use proc_macro2::Span;
use std::collections::HashMap;
use syn::{Error, Ident, LitByteStr, LitStr, Result};

/// A struct of arrays of data used to expand the macro.
#[derive(Default, Clone)]
pub(crate) struct Countries {
    /// The enum variant identifier.
    ident: Vec<Ident>,
    /// The documentation for the given variant.
    doc: Vec<String>,
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
    /// The sovereign (parent) country's identifier, if any, one per country.
    ///
    /// Empty for collections that do not carry parent information.
    parent: Vec<Option<Ident>>,
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
            numeric,
            alpha2_str: Vec::default(),
            alpha2_bytes: Vec::default(),
            alpha3_str: Vec::default(),
            alpha3_bytes: Vec::default(),
            parent: Vec::default(),
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
            numeric: Vec::default(),
            alpha2_str,
            alpha2_bytes,
            alpha3_str: Vec::default(),
            alpha3_bytes: Vec::default(),
            parent: Vec::default(),
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
            numeric: Vec::default(),
            alpha2_str: Vec::default(),
            alpha2_bytes: Vec::default(),
            alpha3_str,
            alpha3_bytes,
            parent: Vec::default(),
        }
    }

    /// Create a new struct-of-arrays collection from the given ipregistry records.
    ///
    /// The `sovereign_country_code` of each [`CountrySovereignty`] record (matched to an
    /// [`IpregistryCountry`] by alpha-2 code) is resolved to the sovereign country's identifier,
    /// and stored in `parent`. Countries without a sovereignty
    /// record, or which are independent, have no parent.
    ///
    /// # Errors
    ///
    /// Returns an error if a sovereign country code does not match any of the given countries.
    #[expect(
        clippy::single_call_fn,
        reason = "Specialized constructor keeping country logic together"
    )]
    pub(crate) fn from_ipregistry(
        countries: &[IpregistryCountry],
        sovereignty: &[CountrySovereignty],
        span: Span,
    ) -> Result<Self> {
        let idents = countries
            .iter()
            .map(|country| (country.alpha2.as_str(), name_to_ident(&country.name_long)))
            .collect::<HashMap<_, _>>();
        let sovereigns = sovereignty
            .iter()
            .map(|record| (record.country_code.as_str(), record))
            .collect::<HashMap<_, _>>();

        let mut retval = Self {
            ident: Vec::with_capacity(countries.len()),
            doc: Vec::with_capacity(countries.len()),
            numeric: Vec::with_capacity(countries.len()),
            alpha2_str: Vec::with_capacity(countries.len()),
            alpha2_bytes: Vec::with_capacity(countries.len()),
            alpha3_str: Vec::with_capacity(countries.len()),
            alpha3_bytes: Vec::with_capacity(countries.len()),
            parent: Vec::with_capacity(countries.len()),
        };

        for country in countries {
            let alpha2 = country.alpha2.as_str();
            let alpha3 = country.alpha3.as_str();
            let name = country.name_long.as_str();

            let parent = sovereigns
                .get(alpha2)
                .and_then(|record| record.sovereign_country_code.as_deref())
                .map(|code| {
                    idents.get(code).cloned().ok_or_else(|| {
                        Error::new(span, format!("Unknown sovereign country code: {code}"))
                    })
                })
                .transpose()?;

            retval.ident.push(name_to_ident(&country.name_long));
            retval.doc.push(format!(" {name} ({alpha2}, {alpha3})"));
            retval.numeric.push(country.numeric_code);
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
            retval.parent.push(parent);
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

    /// Retrieve the parent identifiers, as a slice.
    #[expect(dead_code, reason = "Not yet used in the macro expansion")]
    pub(crate) fn parent(&self) -> &[Option<Ident>] {
        &self.parent
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
fn name_to_ident(name: &str) -> Ident {
    let ident = name
        .trim()
        .to_pascal_case()
        .replace("ÅlandIslands", "AlandIslands")
        .replace("BoliviaPlurinationalStateOf", "Bolivia")
        .replace("TaiwanProvinceOfChina", "Taiwan")
        .replace(
            "CongoDemocraticRepublicOfThe",
            "DemocraticRepublicOfTheCongo",
        )
        .replace(
            "CongoTheDemocraticRepublicOfThe",
            "DemocraticRepublicOfTheCongo",
        )
        .replace("CôteDIvoire", "CoteDIvoire")
        .replace("Curaçao", "Curacao")
        .replace("FalklandIslandsThe", "FalklandIslandsMalvinas")
        .replace("IranIslamicRepublicOf", "Iran")
        .replace("KoreaDemocraticPeopleSRepublicOf", "NorthKorea")
        .replace("KoreaRepublicOf", "SouthKorea")
        .replace("KoreaTheDemocraticPeopleSRepublicOf", "NorthKorea")
        .replace("KoreaTheRepublicOf", "SouthKorea")
        .replace("LaoPeopleSDemocraticRepublic", "Laos")
        .replace("MoldovaRepublicOf", "Moldova")
        .replace("MoldovaTheRepublicOf", "Moldova")
        .replace("MicronesiaFederatedStatesOf", "Micronesia")
        .replace("Naoero", "Nauru")
        .replace("NetherlandsKingdomOfThe", "Netherlands")
        .replace("PalestineStateOf", "Palestine")
        .replace("Réunion", "Reunion")
        .replace("RussianFederation", "Russia")
        .replace("SaintBarthélemy", "SaintBarthelemy")
        .replace("SyrianArabRepublic", "Syria")
        .replace("Türkiye", "Turkey")
        .replace("TanzaniaUnitedRepublicOf", "Tanzania")
        .replace("TanzaniaTheUnitedRepublicOf", "Tanzania")
        .replace(
            "UnitedKingdomOfGreatBritainAndNorthernIreland",
            "UnitedKingdom",
        )
        .replace("VenezuelaBolivarianRepublicOf", "Venezuela")
        .replace("VirginIslandsUnitedKingdom", "BritishVirginIslands")
        .replace("VirginIslandsBritish", "BritishVirginIslands")
        .replace("VirginIslandsUnitedStates", "VirginIslandsUS");

    let ident = ident.strip_suffix("The").unwrap_or(&ident);

    quote::format_ident!("{ident}")
}
