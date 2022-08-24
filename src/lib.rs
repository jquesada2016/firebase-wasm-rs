//! This crate provides bindings to the Firebase v9 JS SDK.
//!
//! There is much to do, and I am only sharing it this early in
//! the hopes that someone might help me generate the remaining
//! API surface. At the bare minimum, a 1:1 API should exist,
//! though idomatic rust wrappers and utilities should exist.
//! A good example of this can be found in the [`UploadTask`](storage::UploadTask)
//! struct, which is a helpful wrapper for converting upload tasks into
//! rust [`streams`](futures::Stream).

#[macro_use]
extern crate clone_macro;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;
pub mod auth;
pub mod firestore;
pub mod storage;
mod types;
