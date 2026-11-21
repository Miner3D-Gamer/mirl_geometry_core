use super::RotateDirections;

#[mirl_macros::derive_all(wincode = false, default = true)]
/// NE SE SW NW
#[allow(missing_docs)]
pub enum ExtendedDirections {
    #[default]
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
const impl RotateDirections for ExtendedDirections {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::NorthEast => Self::SouthEast,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
            Self::NorthWest => Self::NorthEast,
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::NorthEast => Self::NorthWest,
            Self::SouthEast => Self::NorthEast,
            Self::SouthWest => Self::SouthEast,
            Self::NorthWest => Self::SouthWest,
        }
    }
}
