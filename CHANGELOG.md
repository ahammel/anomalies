# Changelog

## [0.2.2] — unreleased

### New

- `#[status(temporary|permanent|persistent)]` attribute on `#[derive(Anomaly)]`. For the two categories without a default status (`interrupted`, `not_found`), you can now set a static status at the derive site instead of writing a manual `impl HasStatus`. Using `#[status(...)]` on a category that already has a default is a compile-time error.

## [0.2.1] — 2026-04-23

### Fixed

- `readme` path in `anomalies/Cargo.toml` corrected to `../README.md` so `cargo publish` can locate it.

## [0.2.0] — 2026-04-23

### Breaking changes

- The sub-trait API (`Unavailable`, `Busy`, `Incorrect`, etc. as traits) has been replaced by a derive macro and a `#[category(...)]` attribute. Migrate by replacing `impl Unavailable for MyError {}` with `#[derive(Anomaly)] #[category(unavailable)] struct MyError;`.
- `Anomaly` now requires `HasCategory + HasStatus` directly instead of the former category sub-traits.

### New

- `#[derive(Anomaly)]` proc macro (in the new `anomalies-derive` crate). Tag your type with `#[category(<name>)]` and the derive generates `HasCategory` and, for categories with a fixed default status, `HasStatus`.
- `anomalies-derive` is published as a separate crate to satisfy Cargo's proc-macro restrictions, but is re-exported by `anomalies` — add only `anomalies` as a dependency.

### Categories with no default status

`interrupted` and `not_found` have no default `HasStatus` because the right answer depends on context. When deriving `Anomaly` for these categories, you must implement `HasStatus` yourself.

## [0.1.0] — 2025-04-07

Initial release.

- `Category` zero-sized marker type and nine built-in categories: `unavailable`, `interrupted`, `busy`, `incorrect`, `forbidden`, `unsupported`, `not_found`, `conflict`, `fault`.
- `Status` enum: `Temporary` and `Permanent`.
- `Anomaly`, `HasCategory`, `HasStatus` traits.
