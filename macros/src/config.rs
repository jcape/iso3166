//! Configuration for the procedural macro.

use proc_macro2::Span;
use std::{
    env,
    path::{Path, PathBuf},
};
use syn::{Error, Expr, ExprLit, Lit, Meta, Result, punctuated::Punctuated, token::Comma};

/// The proc macro configuration.
pub(crate) struct Config {
    /// The path to the JSON file.
    lukes_path: PathBuf,
    /// The configuration span.
    lukes_span: Span,
    /// The path to the ip2location CSV file.
    ip2location_csv: PathBuf,
    /// The span of the ip2location CSV path in the macro input.
    ip2location_span: Span,
    /// Whether UN M49 data should be included.
    _include_m49: bool,
}

impl Config {
    /// Build the configuration from the given macro arguments.
    #[expect(clippy::too_many_lines, clippy::single_call_fn, reason = "Clean code.")]
    pub(crate) fn build(args: &Punctuated<Meta, Comma>) -> Result<Self> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map_err(|_error| Error::new_spanned(args, "CARGO_MANIFEST_DIR not defined"))?;
        let mut lukes_path = Option::<PathBuf>::None;
        let mut include_m49 = Option::<bool>::None;
        let mut lukes_span = Option::<Span>::None;
        let mut ip2location_csv = Option::<PathBuf>::None;
        let mut ip2location_span = Option::<Span>::None;

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
                                    let lp = PathBuf::from(&manifest_dir)
                                        .join("src")
                                        .join(lit_str.value());

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
                        "ip2location_csv" => {
                            if ip2location_csv.is_some() {
                                return Err(Error::new_spanned(
                                    tokens,
                                    "`ip2location_csv` is set twice",
                                ));
                            }

                            match lit {
                                Lit::Str(lit_str) => {
                                    // FIXME: Figure out how to get the relative path to the calling
                                    //        location.
                                    let path = PathBuf::from(&manifest_dir)
                                        .join("src")
                                        .join(lit_str.value());

                                    ip2location_csv = Some(path);
                                    ip2location_span = Some(lit_str.span());
                                }
                                val => {
                                    return Err(Error::new_spanned(
                                        val,
                                        "`ip2location_csv` must be a static string containing CSV data",
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
                                "Unknown attribute {name} is specified; expected one of: `lukes_json`, `ip2location_csv`, `include_m49`",
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
        let ip2location_csv = ip2location_csv
            .ok_or_else(|| Error::new_spanned(args, "`ip2location_csv` was not set"))?;
        let ip2location_span = ip2location_span
            .ok_or_else(|| Error::new_spanned(args, "`ip2location_span` was not set"))?;
        let include_m49 = include_m49.unwrap_or_default();

        Ok(Config {
            lukes_path,
            lukes_span,
            ip2location_csv,
            ip2location_span,
            _include_m49: include_m49,
        })
    }

    /// Retrieve the path to Lukes JSON file.
    pub(crate) fn lukes_path(&self) -> &Path {
        &self.lukes_path
    }

    /// Retrieve the span for the lukes file.
    pub(crate) fn lukes_span(&self) -> &Span {
        &self.lukes_span
    }

    /// Retrieve the path to the ip2location CSV file.
    pub(crate) fn ip2location_csv(&self) -> &Path {
        &self.ip2location_csv
    }

    /// Retrieve the span for the ip2location CSV file.
    pub(crate) fn ip2location_span(&self) -> &Span {
        &self.ip2location_span
    }
}
