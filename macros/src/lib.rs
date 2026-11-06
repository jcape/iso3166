//! Procedural Macros for generating ISO 3166 enumerations and structures.

mod config;
mod countries;
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "Subdivision generation is not wired up yet")
)]
mod subdivisions;

use crate::{config::Config, countries::Countries};
use iso3166_parsers::lukes::Record;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use std::fs::File;
use syn::{Error, Meta, Result, Token, parse::Parser, punctuated::Punctuated};

/// The primary method to output the `Numeric` enum.
#[expect(clippy::single_call_fn, reason = "Clean code, separation of concerns")]
fn numeric(countries: &Countries) -> TokenStream {
    let ident = countries.ident();
    let doc = countries.doc();
    let numeric = countries.numeric();

    let user = Countries::user_numeric();
    let user_doc = user.doc();
    let user_ident = user.ident();
    let user_numeric = user.numeric();

    quote::quote! {
        /// ISO 3166-1 Numeric Country Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
        #[repr(u16)]
        pub enum Numeric {
            #(
                #[doc = #doc]
                #ident = #numeric,
            )*
            #(
                #[doc = #user_doc]
                #user_ident = #user_numeric,
            )*
        }

        impl Numeric {
            /// Try to create a new numeric value from the given 16-bit integer.
            ///
            /// # Errors
            ///
            /// - [`Error::UnknownCode`] when the code is unknown.
            pub const fn from_u16(value: u16) -> Result<Self, Error> {
                match value {
                    #(
                        #numeric => Ok(Self::#ident),
                    )*
                    #(
                        #user_numeric => Ok(Self::#user_ident),
                    )*

                    _ => Err(Error::UnknownCode),
                }
            }

            /// Try to create a new numeric code from the given alpha-2 code.
            ///
            /// Note that user-assigned alpha-2 codes do not have a direct numeric representation.
            ///
            /// # Errors
            ///
            /// - [`Error::UserAssigned`] when attempting to translate to a User-assigned
            ///   Alpha-2 code to a numeric code.
            pub const fn from_alpha2(value: Alpha2) -> Result<Self, Error> {
                match value {
                    #(
                        Alpha2::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Try to create a new numeric code from the given alpha-2 code.
            ///
            /// Note that user-assigned alpha-2 codes do not have a direct numeric representation.
            ///
            /// # Errors
            ///
            /// - [`Error::UserAssigned`] when attempting to translate to a User-assigned
            ///   Alpha-3 code to a numeric code.
            pub const fn from_alpha3(value: Alpha3) -> Result<Self, Error> {
                match value {
                    #(
                        Alpha3::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Determine whether a given enum value represents a user-assigned value.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use iso3166_static::Numeric;
            ///
            /// assert!(!Numeric::UnitedStatesOfAmerica.is_user_assigned());
            /// assert!(Numeric::User900.is_user_assigned());
            /// ```
            pub const fn is_user_assigned(&self) -> bool {
                *self as u16 >= 900 && *self as u16 <= 999
            }
        }

        impl PartialEq<Alpha2> for Numeric {
            fn eq(&self, other: &Alpha2) -> bool {
                match self {
                    #(
                        Self::#ident => *other == Alpha2::#ident,
                    )*
                    _ => false,
                }
            }
        }

        impl PartialEq<Alpha3> for Numeric {
            fn eq(&self, other: &Alpha3) -> bool {
                match self {
                    #(
                        Self::#ident => *other == Alpha3::#ident,
                    )*
                    _ => false,
                }
            }
        }
    }
}

/// The primary function to emit the `Alpha2` enum.
#[expect(clippy::too_many_lines, clippy::single_call_fn, reason = "Clean code.")]
fn alpha2(countries: &Countries) -> TokenStream {
    let ident = countries.ident();
    let doc = countries.doc();
    let alpha2_str = countries.alpha2_str();
    let alpha2_bytes = countries.alpha2_bytes();

    let user = Countries::user_alpha2();
    let user_doc = user.doc();
    let user_ident = user.ident();
    let user_alpha2_str = user.alpha2_str();
    let user_alpha2_bytes = user.alpha2_bytes();

    quote::quote! {
        /// ISO 3166-1 Alpha-2 Country Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
        pub enum Alpha2 {
            #(
                #[doc = #doc]
                #ident,
            )*

            #(
                #[doc = #user_doc]
                #user_ident,
            )*
        }

        impl Alpha2 {
            /// Parse the given alpha-2 string slice into the enum value.
            ///
            /// # Errors
            ///
            /// - [`Error::UnknownCode`] when the string value is not a valid code.
            #[inline]
            pub const fn from_str_slice(value: &str) -> Result<Self, Error> {
                if !value.is_ascii() {
                    return Err(Error::InvalidCharset);
                }

                if value.len() != 2 {
                    return Err(Error::InvalidLength);
                }

                match value.as_bytes() {
                    #(
                        #alpha2_bytes => Ok(Self::#ident),
                    )*

                    #(
                        #user_alpha2_bytes => Ok(Self::#user_ident),
                    )*

                    _ => Err(Error::UnknownCode),
                }
            }

            /// Try to convert the given numeric value into the alpha-2 value.
            ///
            /// # Errors
            ///
            /// - [`Error::UserAssigned`] when the numeric value is unassigned and cannot be
            ///   converted.
            #[inline]
            pub const fn from_numeric(value: Numeric) -> Result<Self, Error> {
                match value {
                    #(
                        Numeric::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Try to convert the given alpha-3 value into the alpha-2 value.
            ///
            /// # Errors
            ///
            /// - [`Error::UserAssigned`] when the numeric value is unassigned and cannot be
            ///   converted.
            #[inline]
            pub const fn from_alpha3(value: Alpha3) -> Result<Self, Error> {
                match value {
                    #(
                        Alpha3::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Get the string representation of the given Alpha-2 code.
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(
                        Self::#ident => #alpha2_str,
                    )*

                    #(
                        Self::#user_ident => #user_alpha2_str,
                    )*
                }
            }

            /// Determine whether a given enum value represents a user-assigned value.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use iso3166_static::Alpha2;
            ///
            /// assert!(!Alpha2::UnitedStatesOfAmerica.is_user_assigned());
            /// assert!(Alpha2::UserXX.is_user_assigned());
            /// ```
            #[inline]
            pub const fn is_user_assigned(&self) -> bool {
                match self {
                    #(
                        Self::#user_ident => true,
                    )*

                    _ => false,
                }
            }
        }

        impl PartialEq<Numeric> for Alpha2 {
            #[inline]
            fn eq(&self, other: &Numeric) -> bool {
                match self {
                    #(
                        Self::#ident => *other == Numeric::#ident,
                    )*
                    _ => false,
                }
            }
        }

        impl PartialEq<Alpha3> for Alpha2 {
            #[inline]
            fn eq(&self, other: &Alpha3) -> bool {
                match self {
                    #(
                        Self::#ident => *other == Alpha3::#ident,
                    )*
                    _ => false,
                }
            }
        }
    }
}

/// The primary method to generate the `Alpha3` enum.
#[expect(clippy::too_many_lines, clippy::single_call_fn, reason = "Clean code.")]
fn alpha3(countries: &Countries) -> TokenStream {
    let doc = countries.doc();
    let ident = countries.ident();
    let alpha3_str = countries.alpha3_str();
    let alpha3_bytes = countries.alpha3_bytes();

    let user = Countries::user_alpha3();
    let user_doc = user.doc();
    let user_ident = user.ident();
    let user_alpha3_str = user.alpha3_str();
    let user_alpha3_bytes = user.alpha3_bytes();

    quote::quote! {
        /// ISO 3166-1 Alpha-3 Country Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
        pub enum Alpha3 {
            #(
                #[doc = #doc]
                #ident,
            )*
            #(
                #[doc = #user_doc]
                #user_ident,
            )*
        }

        impl Alpha3 {
            /// Parse the given alpha-3 string slice into the enum value.
            ///
            /// # Errors
            ///
            /// - [`Error::UnknownCode`] when the string value is not a valid alpha-3 code.
            #[inline]
            pub const fn from_str_slice(value: &str) -> Result<Self, Error> {
                if !value.is_ascii() {
                    return Err(Error::InvalidCharset);
                }

                if value.len() != 3 {
                    return Err(Error::InvalidLength);
                }

                match value.as_bytes() {
                    #(
                        #alpha3_bytes => Ok(Self::#ident),
                    )*

                    #(
                        #user_alpha3_bytes => Ok(Self::#user_ident),
                    )*

                    _ => Err(Error::UnknownCode),
                }
            }

            /// Try to convert the given numeric value into the alpha-3 value.
            ///
            /// # Errors
            ///
            /// - [`Error::UserAssigned`] when the numeric value is unassigned and cannot be
            ///   converted.
            #[inline]
            pub const fn from_numeric(value: Numeric) -> Result<Self, Error> {
                match value {
                    #(
                        Numeric::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Try to convert the given alpha-2 value into the alpha-3 value.
            ///
            /// # Errors
            ///
            /// - [`Error::UserAssigned`] when the numeric value is unassigned and cannot be
            ///   converted.
            #[inline]
            pub const fn from_alpha2(value: Alpha2) -> Result<Self, Error> {
                match value {
                    #(
                        Alpha2::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Get the string representation of the given Alpha-3 code.
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(
                        Self::#ident => #alpha3_str,
                    )*

                    #(
                        Self::#user_ident => #user_alpha3_str,
                    )*
                }
            }

            /// Determine whether a given enum value represents a user-assigned value.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use iso3166_static::Alpha3;
            ///
            /// assert!(!Alpha3::UnitedStatesOfAmerica.is_user_assigned());
            /// assert!(Alpha3::UserZZZ.is_user_assigned());
            /// ```
            #[inline]
            pub const fn is_user_assigned(&self) -> bool {
                match self {
                    #(
                        Self::#user_ident => true,
                    )*

                    _ => false,
                }
            }
        }

        impl PartialEq<Numeric> for Alpha3 {
            #[inline]
            fn eq(&self, other: &Numeric) -> bool {
                match self {
                    #(
                        Self::#ident => *other == Numeric::#ident,
                    )*
                    _ => false,
                }
            }
        }

        impl PartialEq<Alpha2> for Alpha3 {
            #[inline]
            fn eq(&self, other: &Alpha2) -> bool {
                match self {
                    #(
                        Self::#ident => *other == Alpha2::#ident,
                    )*
                    _ => false,
                }
            }
        }
    }
}

/// The top-level fallible generation function.
#[expect(
    clippy::single_call_fn,
    reason = "Separating fallible from infallible functions"
)]
fn try_generate(tokens: TokenStream) -> Result<TokenStream> {
    let config = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(tokens)
        .and_then(|args| Config::build(&args))?;

    let f = File::open(config.lukes_path()).map_err(|error| {
        let message = format!("Could not open JSON path: {error}");
        Error::new(*config.lukes_span(), message)
    })?;

    let mut data = serde_json::from_reader::<_, Vec<Record>>(f).map_err(|error| {
        let message = format!("Could not parse JSON path: {error}");
        Error::new(*config.lukes_span(), message)
    })?;

    data.retain(|record| record.country_code.is_some());
    data.sort_unstable_by(|lhs, rhs| lhs.country_code.cmp(&rhs.country_code));

    let countries = Countries::from_lukes(&data, *config.lukes_span())?;

    let mut retval = quote::quote! {
        /// An enumeration of errors related to ISO 3166 codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
        pub enum Error {
            /// The given value is not a known ISO 3166-1 code.
            UnknownCode,
            /// User-assigned codes cannot be converted between types.
            UserAssigned,
            /// The string length is not a viable code.
            InvalidLength,
            /// The string contains non-ascii characters.
            InvalidCharset,
        }
    };

    let numeric = numeric(&countries);
    let alpha2 = alpha2(&countries);
    let alpha3 = alpha3(&countries);

    retval.extend(numeric);
    retval.extend(alpha2);
    retval.extend(alpha3);

    Ok(retval)
}

/// Generate the relevant types from the provided source data.
///
/// # Panics
///
/// When the generator encountered an error parsing the given input files.
#[expect(clippy::single_call_fn, reason = "Proc macro.")]
#[proc_macro]
pub fn generate(input: TokenStream1) -> TokenStream1 {
    try_generate(input.into())
        .expect("Could not generate output")
        .into()
}
