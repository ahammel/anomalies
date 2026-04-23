use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Ident, parse_macro_input};

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

/// Clone of ::anomalies::status::Status enum for codegen purposes
#[derive(Debug)]
enum DefaultStatus {
    Temporary,
    Permanent,
    // Persistent is never a default status
}

impl DefaultStatus {
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

fn parse_category_string(input: &DeriveInput) -> Result<Category, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("category") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "unavailable" => Ok(Category::Unavailable),
                "interrupted" => Ok(Category::Interrupted),
                "busy" => Ok(Category::Busy),
                "incorrect" => Ok(Category::Incorrect),
                "forbidden" => Ok(Category::Forbidden),
                "unsupported" => Ok(Category::Unsupported),
                "not_found" => Ok(Category::NotFound),
                "conflict" => Ok(Category::Conflict),
                "fault" => Ok(Category::Fault),
                _ => Err(Error::new_spanned(
                    &mode,
                    "expected `unavailable`, `interrupted`, `busy`, \
                        `incorrect`, `forbidden`, `unsupported`, `conflict`, \
                        `not_found`, or `fault`",
                )),
            };
        }
    }
    Err(Error::new_spanned(input, "expected `category` attribue"))
}

#[proc_macro_derive(Anomaly, attributes(category))]
pub fn derive_anomaly(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_anomaly_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_anomaly_inner(input: &DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    let name = &input.ident;
    let category = parse_category_string(input)?;
    let category_ident = format_ident!("{}", format!("{:?}", category));

    let status_impl = match category {
        Category::Unavailable => DefaultStatus::Temporary.derive_for(name),
        Category::Interrupted => quote! {}, // implementer provides status
        Category::Busy => DefaultStatus::Temporary.derive_for(name),
        Category::Incorrect => DefaultStatus::Permanent.derive_for(name),
        Category::Forbidden => DefaultStatus::Permanent.derive_for(name),
        Category::Unsupported => DefaultStatus::Permanent.derive_for(name),
        Category::NotFound => quote! {}, // implementer provides status
        Category::Conflict => DefaultStatus::Permanent.derive_for(name),
        Category::Fault => DefaultStatus::Permanent.derive_for(name),
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
