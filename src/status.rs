//! Retry disposition for anomalies.

/// Whether retrying the same request is likely to succeed.
///
/// `Status` answers the most important operational question a caller has after
/// receiving an error: *should I retry, and if so, when?*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The error is permanent; the caller should not retry without external changes.
    ///
    /// The error reflects a stable condition — bad input, missing permissions, a
    /// missing resource, a logic conflict, or an unsupported operation. The caller
    /// must change something (the request, the credentials, the state of the world)
    /// before trying again.
    Permanent,

    /// The error is temporary; the caller can retry the request to resolve it.
    ///
    /// The error reflects a transient condition: the system is temporarily
    /// unavailable or overloaded. The caller should wait and try again, ideally
    /// with exponential back-off.
    Temporary,

    /// The error was once temporary but persists after retrying.
    ///
    /// This status discourages further retries: the condition has not resolved
    /// itself within a reasonable window and now requires external intervention.
    /// Callers should surface the error rather than continuing to retry.
    Persistent,
}
