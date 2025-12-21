//! Procedural Macros for generating ISO 3166 enumerations and structures

use csv::ReaderBuilder;
use heck::ToPascalCase;
use iso3166_parsers::m49::Record;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use std::io::BufReader;
use syn::{Ident, LitByteStr};

fn ident_from_name(name: &str) -> Ident {
    let ident = name
        .trim()
        .to_pascal_case()
        .replace("Malvinas", "")
        .replace("StateOf", "")
        .replace("ChinaHongKongSpecialAdministrativeRegion", "HongKong")
        .replace("IslamicRepublicOf", "")
        .replace("RepublicOfMoldova", "Moldova")
        .replace("DemocraticPeopleSRepublicOfKorea", "NorthKorea")
        .replace("RepublicOfKorea", "SouthKorea")
        .replace("LaoPeopleSDemocraticRepublic", "Laos")
        .replace("ChinaMacaoSpecialAdministrativeRegion", "Macao")
        .replace("KingdomOfThe", "")
        .replace("SintMaartenDutchPart", "DutchSaintMartin")
        .replace("FederatedStatesOf", "")
        .replace("RussianFederation", "Russia")
        .replace("SaintMartinFrenchPart", "FrenchSaintMartin")
        .replace("SyrianArabRepublic", "Syria")
        .replace(
            "UnitedKingdomOfGreatBritainAndNorthernIreland",
            "UnitedKingdom",
        )
        .replace("UnitedRepublicOfTanzania", "Tanzania")
        .replace("VenezuelaBolivarianRepublicOf", "Venezuela")
        .replace("BoliviaPlurinational", "Bolivia");

    quote::format_ident!("{ident}")
}

#[allow(clippy::too_many_lines)]
fn m49() -> TokenStream {
    let buf_reader = BufReader::new(include_str!("m49.csv").as_bytes());

    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(buf_reader);

    let mut records = reader
        .deserialize::<Record>()
        .map(|result| result.expect("Could not parse M49 record"))
        .collect::<Vec<_>>();

    records.sort_by_cached_key(|val| val.m49_code);

    let (name, code, alpha2, alpha2_bytes, alpha3, alpha3_bytes, ident) = records
        .iter()
        .map(|record| {
            let name = record.country_or_area.as_str();
            let code = record.m49_code;
            let alpha2 = record.alpha2.as_str();
            let alpha2_bytes = LitByteStr::new(alpha2.as_bytes(), Span::call_site());
            let alpha3 = record.alpha3.as_str();
            let alpha3_bytes = LitByteStr::new(alpha3.as_bytes(), Span::call_site());
            let ident = ident_from_name(name);

            (
                name,
                code,
                alpha2,
                alpha2_bytes,
                alpha3,
                alpha3_bytes,
                ident,
            )
        })
        .collect::<(Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>)>();

    quote::quote! {
        /// An enumeration of errors which can be returned while deriving a country code from a
        /// string or integer.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Error {
            /// The string provided is too long to be a country code
            InvalidLength,
            /// The string provided is not a valid ASCII string
            InvalidCharset,
            /// The string provided is not a valid country code string
            UnknownString,
            /// The number provided is not a valid country code
            UnknownCode,
        }

        impl Error {
            /// The enum variant is `InvalidLength`
            pub const fn is_invalid_length(&self) -> bool {
                matches!(self, Self::InvalidLength)
            }

            /// The enum variant is `InvalidCharset`
            pub const fn is_invalid_charset(&self) -> bool {
                matches!(self, Self::InvalidCharset)
            }

            /// The enum variant is `UnknownString`
            pub const fn is_unknown_string(&self) -> bool {
                matches!(self, Self::UnknownString)
            }

            /// The enum variant is `UnknownCode`
            pub const fn is_unknown_code(&self) -> bool {
                matches!(self, Self::UnknownCode)
            }
        }

        impl ::core::fmt::Display for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    Error::InvalidLength => f.pad("Invalid Length"),
                    Error::InvalidCharset => f.pad("Invalid Character Set"),
                    Error::UnknownString => f.pad("Unknown String"),
                    Error::UnknownCode => f.pad("Unknown Code"),
                }
            }
        }

        /// ISO 3166 Countries.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(u16)]
        pub enum Country {
            #(
                #[doc = #name]
                #ident = #code,
            )*
        }

        impl Country {
            /// Create a new country code enum value from the given Alpah2 code string.
            pub const fn try_from_alpha2(s: &::core::primitive::str) -> ::core::result::Result<Self, Error> {
                if s.len() != 2 {
                    return Err(Error::InvalidLength);
                }

                if !s.is_ascii() {
                    return Err(Error::InvalidCharset);
                }

                match s.as_bytes() {
                    #(
                        #alpha2_bytes => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UnknownString),
                }
            }

            /// Create a new country code enum value from the given Alpah3 code string.
            pub const fn try_from_alpha3(s: &::core::primitive::str) -> ::core::result::Result<Self, Error> {
                if s.len() != 3 {
                    return Err(Error::InvalidLength);
                }

                if !s.is_ascii() {
                    return Err(Error::InvalidCharset);
                }

                match s.as_bytes() {
                    #(
                        #alpha3_bytes => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UnknownString),
                }
            }

            /// Create a new country enum from the given u16 value.
            pub const fn try_from_u16(value: ::core::primitive::u16) -> ::core::result::Result<Self, Error> {
                match value {
                    #(
                        #code => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UnknownCode),
                }
            }

            /// Get the Alpha2 code as an uppercase 2-character static string
            pub const fn as_alpha2_str(&self) -> &'static ::core::primitive::str {
                match self {
                    #(
                        Self::#ident => #alpha2,
                    )*
                }
            }

            /// Get the Alpha3 country code as an uppercase static 3-character string
            pub const fn as_alpha3_str(&self) -> &'static ::core::primitive::str {
                match self {
                    #(
                        Self::#ident => #alpha3,
                    )*
                }
            }

            /// Convert this value to a u16
            pub const fn as_u16(&self) -> ::core::primitive::u16 {
                *self as ::core::primitive::u16
            }
        }

        impl ::core::convert::From<Country> for ::core::primitive::u16 {
            fn from(value: Country) -> Self {
                value.as_u16()
            }
        }

        impl ::core::convert::TryFrom<::core::primitive::u16> for Country {
            type Error = Error;

            fn try_from(value: ::core::primitive::u16) -> Result<Self, Self::Error> {
                Self::try_from_u16(value)
            }
        }

        impl ::core::convert::TryFrom<&::core::primitive::str> for Country {
            type Error = Error;

            fn try_from(value: &::core::primitive::str) -> Result<Self, Self::Error> {
                <Self as ::core::str::FromStr>::from_str(value)
            }
        }

        impl ::core::str::FromStr for Country {
            type Err = Error;

            fn from_str(s: &::core::primitive::str) -> ::core::result::Result<Self, Self::Err> {
                let s = s.trim_ascii();

                if !s.is_ascii() {
                    return Err(Error::InvalidCharset);
                }

                let s_bytes = s.as_bytes();
                let s_len = s_bytes.len();
                if s_len > 3 {
                    return Err(Error::InvalidLength);
                }

                let mut src_bytes = [b' '; 3];

                src_bytes[0] = s_bytes[0];
                src_bytes[1] = s_bytes[1];
                if (s_len == 3) {
                    src_bytes[2] = s_bytes[2];
                }
                src_bytes.make_ascii_uppercase();

                #[allow(unsafe_code)]
                // SAFETY: this is safe becase we'e already established the source string is ASCII
                let src = unsafe { ::core::primitive::str::from_utf8_unchecked(&src_bytes) };

                if s_len == 2 {
                    Self::try_from_alpha2(src.trim())
                } else {
                    Self::try_from_alpha3(src)
                }
            }
        }
    }
}

/// Generate data usable for ISO3166 from the M49 dataset.
#[proc_macro]
pub fn generate_m49(_: TokenStream1) -> TokenStream1 {
    m49().into()
}
