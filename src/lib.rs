#![feature(generic_associated_types)]

#![deny(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::cargo_common_metadata
)]
pub mod monad;
pub mod monoid;
pub mod iter;
pub mod transformer;