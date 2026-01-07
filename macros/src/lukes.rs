//! Code generation from the Luke Duncalfe's JSON.

use heck::ToPascalCase;
use iso3166_parsers::lukes::Record;
use proc_macro2::{Span, TokenStream};
use std::{env, fs::File, path::PathBuf};
use syn::{
    Error, Expr, ExprLit, Ident, Lit, LitByteStr, Meta, Result, Token, parse::Parser,
    punctuated::Punctuated, token::Comma,
};

struct Config {
    lukes_path: PathBuf,
    lukes_span: Span,
    _include_m49: bool,
}

impl Config {
    #[allow(clippy::too_many_lines)]
    fn build(args: &Punctuated<Meta, Comma>) -> Result<Self> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| Error::new_spanned(args, "CARGO_MANIFEST_DIR not defined"))?;
        let mut lukes_path = Option::<PathBuf>::None;
        let mut include_m49 = Option::<bool>::None;
        let mut lukes_span = Option::<Span>::None;

        for arg in args {
            match arg {
                Meta::Path(tokens) => {
                    let name = tokens
                        .get_ident()
                        .ok_or_else(|| {
                            syn::Error::new_spanned(tokens, "Must have specified ident")
                        })?
                        .to_string()
                        .to_lowercase();
                    match name.as_str() {
                        "include_m49" => {
                            if include_m49.is_some() {
                                return Err(Error::new_spanned(
                                    tokens,
                                    "`include_m49` is set multiple times",
                                ));
                            }

                            include_m49 = Some(true);
                        }
                        name => {
                            let message = format!(
                                "Unknown attribute {name} is specified; expected one of: `include_m49`."
                            );
                            return Err(Error::new_spanned(tokens, message));
                        }
                    }
                }
                Meta::List(tokens) => {
                    return Err(Error::new_spanned(tokens, "List values not supported"));
                }
                Meta::NameValue(tokens) => {
                    let ident = tokens
                        .path
                        .get_ident()
                        .ok_or_else(|| {
                            syn::Error::new_spanned(tokens, "Must have specified ident")
                        })?
                        .to_string()
                        .to_lowercase();
                    let lit = match &tokens.value {
                        Expr::Lit(ExprLit { lit, .. }) => lit,
                        expr => return Err(Error::new_spanned(expr, "Must be a literal")),
                    };

                    match ident.as_str() {
                        "lukes_json" => {
                            if lukes_path.is_some() {
                                return Err(Error::new_spanned(
                                    tokens,
                                    "`lukes_json` is set twice",
                                ));
                            }

                            match lit {
                                Lit::Str(lit_str) => {
                                    // FIXME: Figure out how to get the relative path to the calling
                                    //        location.
                                    let mut lp = PathBuf::from(&manifest_dir);
                                    lp.push("src");
                                    lp.push(lit_str.value());

                                    lukes_path = Some(lp);
                                    lukes_span = Some(lit_str.span());
                                }
                                val => {
                                    return Err(Error::new_spanned(
                                        val,
                                        "`lukes_json` must be a static string containing JSON data",
                                    ));
                                }
                            }
                        }
                        "include_m49" => {
                            if include_m49.is_some() {
                                return Err(Error::new_spanned(
                                    tokens,
                                    "`include_m49` is set multiple times",
                                ));
                            }

                            match lit {
                                Lit::Bool(lit_bool) => {
                                    include_m49 = Some(lit_bool.value());
                                }
                                val => {
                                    return Err(Error::new_spanned(
                                        val,
                                        "`include_m49` must be a boolean when set.",
                                    ));
                                }
                            }
                        }
                        name => {
                            let message = format!(
                                "Unknown attribute {name} is specified; expected one of: `lukes_json`, `include_m49`",
                            );
                            return Err(Error::new_spanned(tokens, message));
                        }
                    }
                }
            }
        }

        let lukes_path =
            lukes_path.ok_or_else(|| Error::new_spanned(args, "`lukes_path` was not set"))?;
        let lukes_span =
            lukes_span.ok_or_else(|| Error::new_spanned(args, "`lukes_span` was not set"))?;
        let include_m49 = include_m49.unwrap_or_default();

        Ok(Config {
            lukes_path,
            lukes_span,
            _include_m49: include_m49,
        })
    }
}

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

#[allow(clippy::too_many_lines)]
fn numeric(config: &Config, data: &[Record]) -> Result<TokenStream> {
    let mut ident = Vec::new();
    let mut code = Vec::new();
    let mut doc = Vec::new();
    let mut name = Vec::new();
    let mut alpha2 = Vec::new();
    let mut alpha3 = Vec::new();

    let mut records = data
        .iter()
        .filter(|&record| record.country_code.is_some())
        .collect::<Vec<_>>();
    records.sort_by_cached_key(|&record| record.country_code.as_deref());

    for record in records {
        if let Some(cc) = record.country_code.as_deref()
            && let Some(n) = record.name.as_deref()
            && let Some(a2) = record.alpha_2.as_deref()
            && let Some(a3) = record.alpha_3.as_deref()
        {
            let id = name_to_ident(n);
            let c = cc
                .parse::<u16>()
                .map_err(|err| Error::new(config.lukes_span, err.to_string()))?;
            let d = format!(" {n} ({a2}, {a3})");

            ident.push(id);
            code.push(c);
            doc.push(d);
            name.push(n);
            alpha2.push(a2);
            alpha3.push(a3);
        }
    }

    let (user_ident, user_code, user_doc) = (900..999u16)
        .map(|i| {
            (
                quote::format_ident!("User{i}"),
                i,
                format!("User-assigned {i}"),
            )
        })
        .collect::<(Vec<Ident>, Vec<u16>, Vec<String>)>();

    let mut retval = quote::quote! {
        /// ISO 3166-1 Numeric Country Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(u16)]
        pub enum Numeric {
            #(
                #[doc = #doc]
                #ident = #code,
            )*
            #(
                #[doc = #user_doc]
                #user_ident = #user_code,
            )*
        }
    };

    retval.extend(quote::quote! {
        impl Numeric {
            /// Try to create a new numeric value from the given 16-bit integer.
            ///
            /// # Errors
            ///
            /// - [`Error::UnknownCode`] when the code is unknown.
            pub const fn from_u16(value: u16) -> Result<Self, Error> {
                match value {
                    #(
                        #code => Ok(Self::#ident),
                    )*
                    #(
                        #user_code => Ok(Self::#user_ident),
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
    });

    Ok(retval)
}

#[allow(clippy::too_many_lines)]
fn alpha2(data: &[Record]) -> TokenStream {
    let mut ident = Vec::new();
    let mut doc = Vec::new();
    let mut name = Vec::new();
    let mut alpha2 = Vec::new();
    let mut alpha2_bytes = Vec::new();

    let mut records = data
        .iter()
        .filter(|&record| record.country_code.is_some())
        .collect::<Vec<_>>();
    records.sort_by_cached_key(|&record| record.country_code.as_deref());

    for record in records {
        if let Some(cc) = record.country_code.as_deref()
            && let Some(n) = record.name.as_deref()
            && let Some(a2) = record.alpha_2.as_deref()
            && let Some(a3) = record.alpha_3.as_deref()
        {
            let id = name_to_ident(n);
            let d = format!(" {n} ({cc}, {a2}, {a3})");
            let bytes = LitByteStr::new(a2.as_bytes(), Span::mixed_site());

            ident.push(id);
            doc.push(d);
            name.push(n);
            alpha2.push(a2);
            alpha2_bytes.push(bytes);
        }
    }

    // User-generated Alpha-2
    let mut user_doc = Vec::new();
    let mut user_ident = Vec::new();
    let mut user_alpha2 = Vec::new();
    let mut user_alpha2_bytes = Vec::new();

    for user in [
        "AA", "QM", "QN", "QO", "QP", "QQ", "QR", "QS", "QT", "QU", "QV", "QW", "QX", "QY", "QZ",
        "XA", "XB", "XC", "XD", "XE", "XF", "XG", "XH", "XI", "XJ", "XK", "XL", "XM", "XN", "XO",
        "XP", "XQ", "XR", "XS", "XT", "XU", "XV", "XW", "XX", "XY", "XZ", "ZZ",
    ] {
        let d = format!(" User-assigned {user}");
        let id = quote::format_ident!("User{user}");
        let bytes = LitByteStr::new(user.as_bytes(), Span::mixed_site());

        user_doc.push(d);
        user_ident.push(id);
        user_alpha2.push(user);
        user_alpha2_bytes.push(bytes);
    }

    quote::quote! {
        /// ISO 3166-1 Alpha-2 Country Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
            pub const fn from_alpha3(value: Alpha3) -> Result<Self, Error> {
                match value {
                    #(
                        Alpha3::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Get the string representation of the given Alpha-2 code.
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(
                        Self::#ident => #alpha2,
                    )*

                    #(
                        Self::#user_ident => #user_alpha2,
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

fn make_user_alpha3(pos1: char, pos2: char, pos3: char) -> (Ident, String, String) {
    let mut alpha3 = String::new();
    alpha3.push(pos1);
    alpha3.push(pos2);
    alpha3.push(pos3);

    let ident = quote::format_ident!("User{alpha3}");
    let doc = format!(" User-assigned {alpha3}");

    (ident, doc, alpha3)
}

#[allow(clippy::too_many_lines)]
fn alpha3(data: &[Record]) -> TokenStream {
    let mut ident = Vec::new();
    let mut doc = Vec::new();
    let mut name = Vec::new();
    let mut alpha3 = Vec::new();
    let mut alpha3_bytes = Vec::new();

    let mut records = data
        .iter()
        .filter(|&record| record.country_code.is_some())
        .collect::<Vec<_>>();
    records.sort_by_cached_key(|&record| record.country_code.as_deref());

    for record in records {
        if let Some(cc) = record.country_code.as_deref()
            && let Some(n) = record.name.as_deref()
            && let Some(a2) = record.alpha_2.as_deref()
            && let Some(a3) = record.alpha_3.as_deref()
        {
            let id = name_to_ident(n);
            let d = format!(" {n} ({cc}, {a2}, {a3})");
            let bytes = LitByteStr::new(a3.as_bytes(), Span::mixed_site());

            ident.push(id);
            doc.push(d);
            name.push(n);
            alpha3.push(a3);
            alpha3_bytes.push(bytes);
        }
    }

    // User-generated Alpha-2
    let mut user_doc = Vec::new();
    let mut user_ident = Vec::new();
    let mut user_alpha3 = Vec::new();
    let mut user_alpha3_bytes = Vec::new();

    // AAA-AAZ
    for code in 'A'..='Z' {
        let (id, d, a3) = make_user_alpha3('A', 'A', code);
        let a3b = LitByteStr::new(a3.as_bytes(), Span::mixed_site());
        user_ident.push(id);
        user_doc.push(d);
        user_alpha3.push(a3);
        user_alpha3_bytes.push(a3b);
    }

    // QMA-QZZ
    for code in 'M'..='Z' {
        for code2 in 'A'..='Z' {
            let (id, d, a3) = make_user_alpha3('Q', code, code2);
            let a3b = LitByteStr::new(a3.as_bytes(), Span::mixed_site());
            user_ident.push(id);
            user_doc.push(d);
            user_alpha3.push(a3);
            user_alpha3_bytes.push(a3b);
        }
    }

    // XAA - XZZ
    for code in 'A'..='Z' {
        for code2 in 'A'..='Z' {
            let (id, d, a3) = make_user_alpha3('X', code, code2);
            let a3b = LitByteStr::new(a3.as_bytes(), Span::mixed_site());
            user_ident.push(id);
            user_doc.push(d);
            user_alpha3.push(a3);
            user_alpha3_bytes.push(a3b);
        }
    }

    // ZZA-ZZZ
    for code in 'A'..='Z' {
        let (id, d, a3) = make_user_alpha3('Z', 'Z', code);
        let a3b = LitByteStr::new(a3.as_bytes(), Span::mixed_site());
        user_ident.push(id);
        user_doc.push(d);
        user_alpha3.push(a3);
        user_alpha3_bytes.push(a3b);
    }

    quote::quote! {
        /// ISO 3166-1 Alpha-3 Country Codes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
            pub const fn from_alpha2(value: Alpha2) -> Result<Self, Error> {
                match value {
                    #(
                        Alpha2::#ident => Ok(Self::#ident),
                    )*
                    _ => Err(Error::UserAssigned),
                }
            }

            /// Get the string representation of the given Alpha-3 code.
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #(
                        Self::#ident => #alpha3,
                    )*

                    #(
                        Self::#user_ident => #user_alpha3,
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

fn try_generate(tokens: TokenStream) -> Result<TokenStream> {
    let config = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(tokens)
        .and_then(|args| Config::build(&args))?;

    let f = File::open(&config.lukes_path).map_err(|error| {
        let message = format!("Could not open JSON path: {error}");
        Error::new(config.lukes_span, message)
    })?;

    let data = serde_json::from_reader::<_, Vec<Record>>(f).map_err(|error| {
        let message = format!("Could not parse JSON path: {error}");
        Error::new(config.lukes_span, message)
    })?;

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

    let numeric = numeric(&config, &data)?;
    let alpha2 = alpha2(&data);
    let alpha3 = alpha3(&data);

    retval.extend(numeric);
    retval.extend(alpha2);
    retval.extend(alpha3);

    Ok(retval)
}

pub(crate) fn generate(tokens: TokenStream) -> TokenStream {
    try_generate(tokens).expect("Could not generate output")
}
