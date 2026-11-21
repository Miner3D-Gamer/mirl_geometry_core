#![allow(clippy::modulo_one)]

#[mirl_macros::derive_all(default = true)]
/// None
pub enum SpecialDirections {
    #[allow(missing_docs)]
    #[default]
    None,
}
