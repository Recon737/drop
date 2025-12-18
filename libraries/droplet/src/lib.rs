#![deny(clippy::all)]
#![feature(impl_trait_in_bindings)]

pub mod file_utils;
pub mod ssl;
pub mod versions;
pub mod manifest;

#[cfg(test)]
pub mod tests;
