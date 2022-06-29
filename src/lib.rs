#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
//! The Pokémon Service allows you to retrieve information about Pokémon species.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Errors that can occur when calling the service.
pub mod error;
/// Input structures for operations.
pub mod input;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
/// Operation handlers definition and implementation.
pub mod operation_handler;
/// A registry of your service's operations.
pub mod operation_registry;
mod operation_ser;
/// Output structures for operations.
pub mod output;
mod server_operation_handler_trait;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::byte_stream::AggregatedBytes;
    pub use aws_smithy_http::byte_stream::ByteStream;
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_types::DateTime;
}
