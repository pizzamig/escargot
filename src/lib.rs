//! # Escargot: A Cargo API
//!
//! ## Install
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! escargot = "0.4"
//! ```
//!
//! Features:
//! - `print` for logged output to be printed instead, generally for test writing.
//!
//! ## Why escargot
//!
//! Compared to depending on `cargo`:
//! - Faster compile times.
//! - Simpler API.
//! - Better interop with projects relying on other cargo versions.
//! - Probably slower execution, especially on platforms without an optimized `fork` (e.g. Windows).
//!
//! ## Relevant crates
//!
//! Other related crates:
//! * [cargo](https://crates.io/crates/cargo) for the real thing
//! * [cargo-metadata](https://crates.io/crates/cargo_metadata) for a similar project specifically geared to the `metadata` subcommand.
//!
//! # Example
//!
//! ```rust
//! extern crate escargot;
//! extern crate assert_fs;
//!
//! let temp = assert_fs::TempDir::new().unwrap();
//! escargot::CargoBuild::new()
//!     .bin("bin")
//!     .current_release()
//!     .current_target()
//!     .manifest_path("tests/fixtures/bin/Cargo.toml")
//!     .target_dir(temp.path())
//!     .exec()
//!     .unwrap();
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod build;
pub use build::*;
mod cargo;
pub use cargo::*;
mod msg;
pub use msg::*;
mod run;
pub use run::*;
#[cfg(feature = "test_unstable")]
mod test;
#[cfg(feature = "test_unstable")]
pub use test::*;

pub mod error;
pub mod format;
