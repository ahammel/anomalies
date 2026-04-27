use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DeriveInput, Error, Fields, Ident, parse_macro_input};

#[derive(Debug)]
enum Category {
    /// The requested resource or service is not currently reachable.
    ///
    /// Unlike [`Busy`], `Unavailable` implies the dependency itself is absent or
    /// down — not merely overloaded. The request may succeed later if the dependency
    /// recovers.
    ///
    /// Default status: [`Status::Temporary`](crate::status::Status::Temporary).
    Unavailable,

    /// The operation was cut short before it could complete.
    ///
    /// The system was reachable and processing the request, but something (a timeout,
    /// a cancellation signal, a network reset) stopped it mid-flight. Whether the
    /// caller should retry depends on whether the partial work was committed; there is
    /// no universal default status for this category.
    Interrupted,

    /// The system is reachable but overloaded and cannot accept more work right now.
    ///
    /// Unlike [`Unavailable`], the dependency is up and healthy — it just has more
    /// demand than capacity at this moment. Callers should back off and retry.
    ///
    /// Default status: [`Status::Temporary`](crate::status::Status::Temporary).
    Busy,

    /// The request itself is malformed or invalid.
    ///
    /// The problem is with what was asked, not with the system's current state.
    /// Retrying the same request unchanged will not help.
    ///
    /// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
    Incorrect,

    /// The caller does not have permission to perform the operation.
    ///
    /// The request was understood but authorization was denied. Retrying with the same
    /// credentials will not help; the caller needs elevated privileges or a different
    /// identity.
    ///
    /// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
    Forbidden,

    /// The operation is not supported by this implementation.
    ///
    /// The system understood the request but has no capability to fulfill it. This is
    /// a permanent condition: the feature simply does not exist.
    ///
    /// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
    Unsupported,

    /// The requested resource does not exist.
    ///
    /// The identifier or path is valid but points to nothing. Whether this is
    /// permanent depends on context (e.g. a deleted record vs. a race with a
    /// concurrent create), so there is no universal default status for this category.
    NotFound,

    /// The operation cannot be applied because it conflicts with existing state.
    ///
    /// Typically a uniqueness violation, an optimistic-lock mismatch, or a
    /// precondition failure. Retrying the same request unchanged will not resolve
    /// the conflict; the caller must reconcile the state difference first.
    ///
    /// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
    Conflict,

    /// An internal error that is the system's fault, not the caller's.
    ///
    /// The request was valid but something went wrong internally — a bug, a failed
    /// invariant, an unexpected state. The caller cannot fix this by changing their
    /// request.
    ///
    /// Default status: [`Status::Permanent`](crate::status::Status::Permanent).
    Fault,
}

/// A status value for codegen — either from a category default or an explicit `#[status(...)]`.
#[derive(Debug)]
enum StatusSpec {
    Temporary,
    Permanent,
    Persistent,
}

impl StatusSpec {
    fn derive_for(&self, name: &Ident) -> proc_macro2::TokenStream {
        let status = format_ident!("{}", format!("{:?}", self));
        quote! {
            impl ::anomalies::anomaly::HasStatus for #name {
                fn status(&self) -> ::anomalies::status::Status {
                    ::anomalies::status::Status::#status
                }
            }
        }
    }
}

fn parse_category_attrs(attrs: &[syn::Attribute]) -> Result<Option<Category>, Error> {
    for attr in attrs {
        if attr.path().is_ident("category") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "unavailable" => Ok(Some(Category::Unavailable)),
                "interrupted" => Ok(Some(Category::Interrupted)),
                "busy" => Ok(Some(Category::Busy)),
                "incorrect" => Ok(Some(Category::Incorrect)),
                "forbidden" => Ok(Some(Category::Forbidden)),
                "unsupported" => Ok(Some(Category::Unsupported)),
                "not_found" => Ok(Some(Category::NotFound)),
                "conflict" => Ok(Some(Category::Conflict)),
                "fault" => Ok(Some(Category::Fault)),
                _ => Err(Error::new_spanned(
                    &mode,
                    "expected `unavailable`, `interrupted`, `busy`, \
                        `incorrect`, `forbidden`, `unsupported`, `conflict`, \
                        `not_found`, or `fault`",
                )),
            };
        }
    }
    Ok(None)
}

fn parse_status_attrs(attrs: &[syn::Attribute]) -> Result<Option<(Ident, StatusSpec)>, Error> {
    for attr in attrs {
        if attr.path().is_ident("status") {
            let mode: Ident = attr.parse_args()?;
            let spec = match mode.to_string().as_str() {
                "temporary" => StatusSpec::Temporary,
                "permanent" => StatusSpec::Permanent,
                "persistent" => StatusSpec::Persistent,
                _ => {
                    return Err(Error::new_spanned(
                        &mode,
                        "expected `temporary`, `permanent`, or `persistent`",
                    ));
                }
            };
            return Ok(Some((mode, spec)));
        }
    }
    Ok(None)
}

/// Returns the `transparent` ident if `#[anomaly(transparent)]` is present.
fn parse_transparent_attr(attrs: &[syn::Attribute]) -> Result<Option<Ident>, Error> {
    for attr in attrs {
        if attr.path().is_ident("anomaly") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "transparent" => Ok(Some(mode)),
                _ => Err(Error::new_spanned(&mode, "expected `transparent`")),
            };
        }
    }
    Ok(None)
}

fn category_default_status(category: &Category) -> Option<StatusSpec> {
    match category {
        Category::Unavailable | Category::Busy => Some(StatusSpec::Temporary),
        Category::Incorrect
        | Category::Forbidden
        | Category::Unsupported
        | Category::Conflict
        | Category::Fault => Some(StatusSpec::Permanent),
        Category::Interrupted | Category::NotFound => None,
    }
}

/// Returns `(match_pattern, inner_binding)` for a transparent variant.
fn transparent_pattern(
    vname: &Ident,
    fields: &Fields,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), Error> {
    let err = || {
        Error::new_spanned(
            vname,
            "`#[anomaly(transparent)]` requires exactly one field",
        )
    };
    match fields {
        Fields::Unit => Err(err()),
        Fields::Unnamed(f) if f.unnamed.len() == 1 => {
            Ok((quote! { Self::#vname(inner) }, quote! { inner }))
        }
        Fields::Unnamed(_) => Err(err()),
        Fields::Named(f) if f.named.len() == 1 => {
            let fname = f.named[0].ident.as_ref().unwrap();
            Ok((quote! { Self::#vname { #fname } }, quote! { #fname }))
        }
        Fields::Named(_) => Err(err()),
    }
}

#[proc_macro_derive(Anomaly, attributes(category, status, anomaly))]
pub fn derive_anomaly(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_anomaly_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_anomaly_inner(input: &DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let name = &input.ident;
    match &input.data {
        Data::Struct(_) => derive_struct(name, input),
        Data::Enum(data) => derive_enum(name, data),
        Data::Union(_) => Err(Error::new_spanned(
            input,
            "#[derive(Anomaly)] is not supported for unions",
        )),
    }
}

fn derive_struct(name: &Ident, input: &DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let category = parse_category_attrs(&input.attrs)?
        .ok_or_else(|| Error::new_spanned(input, "expected `#[category(...)]` attribute"))?;
    let category_ident = format_ident!("{}", format!("{:?}", category));
    let explicit_status = parse_status_attrs(&input.attrs)?;
    let default_status = category_default_status(&category);

    let status_impl = match (default_status, explicit_status) {
        (Some(_), Some((status_ident, _))) => {
            return Err(Error::new_spanned(
                &status_ident,
                "`#[status(...)]` is only valid for categories without a default (`interrupted`, `not_found`)",
            ));
        }
        (Some(default), None) => default.derive_for(name),
        (None, Some((_, spec))) => spec.derive_for(name),
        (None, None) => quote! {},
    };

    Ok(quote! {
        impl ::anomalies::anomaly::Anomaly for #name {}

        impl ::anomalies::anomaly::HasCategory for #name {
            fn category(&self) -> ::anomalies::category::Category {
                ::anomalies::category::#category_ident
            }
        }

        #status_impl
    })
}

fn derive_enum(name: &Ident, data: &DataEnum) -> Result<proc_macro2::TokenStream, Error> {
    let mut category_arms = Vec::new();
    let mut status_arms = Vec::new();

    for variant in &data.variants {
        let vname = &variant.ident;
        let transparent = parse_transparent_attr(&variant.attrs)?;
        let category = parse_category_attrs(&variant.attrs)?;

        match (transparent, category) {
            // Conflict: both transparent and category on the same variant.
            (Some(t_ident), Some(_)) => {
                return Err(Error::new_spanned(
                    &t_ident,
                    "`#[anomaly(transparent)]` cannot be combined with `#[category(...)]`",
                ));
            }

            // Transparent: delegate category and status to the inner field.
            (Some(_), None) => {
                if let Some((s_ident, _)) = parse_status_attrs(&variant.attrs)? {
                    return Err(Error::new_spanned(
                        &s_ident,
                        "`#[anomaly(transparent)]` cannot be combined with `#[status(...)]`",
                    ));
                }
                let (pattern, inner) = transparent_pattern(vname, &variant.fields)?;
                category_arms.push(quote! {
                    #pattern => ::anomalies::anomaly::HasCategory::category(#inner)
                });
                status_arms.push(quote! {
                    #pattern => ::anomalies::anomaly::HasStatus::status(#inner)
                });
            }

            // Categorized: generate static arms from #[category(...)] / #[status(...)].
            (None, Some(cat)) => {
                let category_ident = format_ident!("{}", format!("{:?}", cat));
                let explicit_status = parse_status_attrs(&variant.attrs)?;
                let default_status = category_default_status(&cat);

                let status_spec = match (default_status, explicit_status) {
                    (Some(_), Some((status_ident, _))) => {
                        return Err(Error::new_spanned(
                            &status_ident,
                            "`#[status(...)]` is only valid for categories without a default (`interrupted`, `not_found`)",
                        ));
                    }
                    (Some(default), None) => default,
                    (None, Some((_, spec))) => spec,
                    (None, None) => {
                        return Err(Error::new_spanned(
                            vname,
                            "variant has no default status; add `#[status(temporary|permanent|persistent)]`",
                        ));
                    }
                };

                let pattern = match &variant.fields {
                    Fields::Unit => quote! { Self::#vname },
                    Fields::Unnamed(_) => quote! { Self::#vname(..) },
                    Fields::Named(_) => quote! { Self::#vname { .. } },
                };

                let status_ident = format_ident!("{}", format!("{:?}", status_spec));
                category_arms.push(quote! { #pattern => ::anomalies::category::#category_ident });
                status_arms.push(quote! { #pattern => ::anomalies::status::Status::#status_ident });
            }

            // Neither: missing annotation.
            (None, None) => {
                return Err(Error::new_spanned(
                    vname,
                    "expected `#[category(...)]` or `#[anomaly(transparent)]` on variant",
                ));
            }
        }
    }

    Ok(quote! {
        impl ::anomalies::anomaly::Anomaly for #name {}

        impl ::anomalies::anomaly::HasCategory for #name {
            fn category(&self) -> ::anomalies::category::Category {
                match self {
                    #(#category_arms,)*
                }
            }
        }

        impl ::anomalies::anomaly::HasStatus for #name {
            fn status(&self) -> ::anomalies::status::Status {
                match self {
                    #(#status_arms,)*
                }
            }
        }
    })
}
