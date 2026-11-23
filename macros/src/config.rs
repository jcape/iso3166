//! Configuration for the procedural macro.

use proc_macro2::Span;
use std::{
    env,
    path::{Path, PathBuf},
};
use syn::{
    Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, punctuated::Punctuated, token::Comma,
};

/// A path to a data file, with the span of the path in the macro input.
pub(crate) struct Source {
    /// The path to the file.
    path: PathBuf,
    /// The span of the path in the macro input.
    span: Span,
}

impl Source {
    /// Retrieve the path to the file.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// Retrieve the span of the path in the macro input.
    pub(crate) fn span(&self) -> Span {
        self.span
    }
}

/// The proc macro configuration.
pub(crate) struct Config {
    /// The ipregistry `countries.csv` file.
    ipregistry_countries: Source,
    /// The ipregistry `countries-sovereignty.csv` file.
    ipregistry_sovereignty: Source,
    /// The ipregistry `subdivisions.csv` file.
    ipregistry_subdivisions: Source,
    /// The ip2location CSV file.
    ip2location: Source,
    /// Whether UN M49 data should be included.
    _include_m49: bool,
}

/// Store a string-literal path argument (relative to the calling crate's `src` directory) into
/// the given slot, failing if it is already set or is not a string.
fn set_source(
    slot: &mut Option<Source>,
    name: &str,
    manifest_dir: &str,
    tokens: &MetaNameValue,
    lit: &Lit,
) -> Result<()> {
    if slot.is_some() {
        return Err(Error::new_spanned(tokens, format!("`{name}` is set twice")));
    }

    match lit {
        Lit::Str(lit_str) => {
            // FIXME: Figure out how to get the relative path to the calling location.
            let path = PathBuf::from(manifest_dir)
                .join("src")
                .join(lit_str.value());

            *slot = Some(Source {
                path,
                span: lit_str.span(),
            });
            Ok(())
        }
        val => Err(Error::new_spanned(
            val,
            format!("`{name}` must be a static string containing a file path"),
        )),
    }
}

/// Require that the given argument was set.
fn require(slot: Option<Source>, name: &str, args: &Punctuated<Meta, Comma>) -> Result<Source> {
    slot.ok_or_else(|| Error::new_spanned(args, format!("`{name}` was not set")))
}

impl Config {
    /// Build the configuration from the given macro arguments.
    #[expect(clippy::too_many_lines, clippy::single_call_fn, reason = "Clean code.")]
    pub(crate) fn build(args: &Punctuated<Meta, Comma>) -> Result<Self> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map_err(|_error| Error::new_spanned(args, "CARGO_MANIFEST_DIR not defined"))?;
        let mut include_m49 = Option::<bool>::None;
        let mut ipregistry_countries = Option::<Source>::None;
        let mut ipregistry_sovereignty = Option::<Source>::None;
        let mut ipregistry_subdivisions = Option::<Source>::None;
        let mut ip2location = Option::<Source>::None;

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
                        "ipregistry_countries_csv" => set_source(
                            &mut ipregistry_countries,
                            "ipregistry_countries_csv",
                            &manifest_dir,
                            tokens,
                            lit,
                        )?,
                        "ipregistry_countries_sovereignty_csv" => set_source(
                            &mut ipregistry_sovereignty,
                            "ipregistry_countries_sovereignty_csv",
                            &manifest_dir,
                            tokens,
                            lit,
                        )?,
                        "ipregistry_subdivisions_csv" => set_source(
                            &mut ipregistry_subdivisions,
                            "ipregistry_subdivisions_csv",
                            &manifest_dir,
                            tokens,
                            lit,
                        )?,
                        "ip2location_csv" => set_source(
                            &mut ip2location,
                            "ip2location_csv",
                            &manifest_dir,
                            tokens,
                            lit,
                        )?,
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
                                "Unknown attribute {name} is specified; expected one of: `ipregistry_countries_csv`, `ipregistry_countries_sovereignty_csv`, `ipregistry_subdivisions_csv`, `ip2location_csv`, `include_m49`",
                            );
                            return Err(Error::new_spanned(tokens, message));
                        }
                    }
                }
            }
        }

        Ok(Config {
            ipregistry_countries: require(ipregistry_countries, "ipregistry_countries_csv", args)?,
            ipregistry_sovereignty: require(
                ipregistry_sovereignty,
                "ipregistry_countries_sovereignty_csv",
                args,
            )?,
            ipregistry_subdivisions: require(
                ipregistry_subdivisions,
                "ipregistry_subdivisions_csv",
                args,
            )?,
            ip2location: require(ip2location, "ip2location_csv", args)?,
            _include_m49: include_m49.unwrap_or_default(),
        })
    }

    /// Retrieve the ipregistry `countries.csv` source.
    pub(crate) fn ipregistry_countries(&self) -> &Source {
        &self.ipregistry_countries
    }

    /// Retrieve the ipregistry `countries-sovereignty.csv` source.
    pub(crate) fn ipregistry_sovereignty(&self) -> &Source {
        &self.ipregistry_sovereignty
    }

    /// Retrieve the ipregistry `subdivisions.csv` source.
    #[expect(
        dead_code,
        reason = "Subdivisions are not yet generated from ipregistry"
    )]
    pub(crate) fn ipregistry_subdivisions(&self) -> &Source {
        &self.ipregistry_subdivisions
    }

    /// Retrieve the ip2location CSV source.
    pub(crate) fn ip2location(&self) -> &Source {
        &self.ip2location
    }
}
