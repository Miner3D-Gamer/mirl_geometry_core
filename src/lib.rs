//! A bare bone crate defining minimal geometric related items
#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![feature(derive_const)]
#![feature(const_clone)]
#![feature(const_cmp)]

/// Directional stuff -> NESW, N NE E SE S SW W NW
pub mod directions;

/// A position within a string
pub mod text_position;
